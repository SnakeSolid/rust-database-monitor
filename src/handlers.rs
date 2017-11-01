use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;

use serde_json;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::mime::SubLevel;
use iron::mime::TopLevel;
use iron::Request;
use iron::Response;
use iron::status;

use state::DatabaseRow;
use state::State;


#[derive(Deserialize, Debug, Clone)]
struct DatabasesRequest {
    query: String,
}


#[derive(Serialize, Debug, Clone)]
struct DatabaseInfo {
    database_name: String,
    collation_name: String,
    role_name: String,
    server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_description: Option<String>,
    last_update: Option<i64>,
}


#[derive(Serialize, Debug, Clone)]
struct DatabasesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    databases: Option<Vec<DatabaseInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    ok: bool,
}


pub struct DatabasesHandler {
    state: State,
}


impl DatabasesResponse {
    fn ok(databases: Vec<DatabaseInfo>) -> DatabasesResponse {
        DatabasesResponse {
            databases: Some(databases),
            message: None,
            ok: true,
        }
    }

    fn err(message: &str) -> DatabasesResponse {
        DatabasesResponse {
            databases: None,
            message: Some(message.into()),
            ok: false,
        }
    }
}


impl DatabasesHandler {
    pub fn new(state: State) -> DatabasesHandler {
        DatabasesHandler { state: state }
    }

    fn range_databases<S>(query: S, database_list: Vec<DatabaseRow>) -> Vec<DatabaseInfo>
    where
        S: Into<String>,
    {
        let query = query.into();
        let mut ranks = HashMap::new();

        for part in query.split(|c| " _-".contains(c)).filter(|e| !e.is_empty()) {
            for database in &database_list {
                if database.search_doc.contains(part) {
                    let count = ranks.entry(database).or_insert(0);

                    *count += part.len();
                }
            }
        }

        let mut ranks: Vec<_> = ranks.into_iter().collect();

        ranks.sort_by(Self::compare_databases);

        let result: Vec<_> = ranks
            .into_iter()
            .take(30)
            .map(|(db, _)| {
                DatabaseInfo {
                    database_name: db.database_name.clone(),
                    collation_name: db.database_collate.clone(),
                    role_name: db.database_owner.clone(),
                    server_name: db.server_name.clone(),
                    server_description: db.server_description.clone(),
                    last_update: Some(db.last_update),
                }
            })
            .collect();

        result
    }

    fn compare_databases(a: &(&DatabaseRow, usize), b: &(&DatabaseRow, usize)) -> Ordering {
        let &(a_db, a_rank) = a;
        let &(b_db, b_rank) = b;

        if a_rank < b_rank {
            return Ordering::Greater;
        } else if a_rank > b_rank {
            return Ordering::Less;
        } else if a_db.database_name < b_db.database_name {
            return Ordering::Greater;
        } else if a_db.database_name > b_db.database_name {
            return Ordering::Less;
        } else if a_db.server_name < b_db.server_name {
            return Ordering::Greater;
        } else if a_db.database_name > b_db.database_name {
            return Ordering::Less;
        } else {
            Ordering::Equal
        }
    }
}

macro_rules! or_bad_request {
    ($param: expr, $message: tt) => {
        match $param {
            Ok(value) => value,
            Err(err) => {
                warn!("{}: {}", $message, err);

                return Ok(Response::with((status::BadRequest, $message)));
            }
        }
    }
}

macro_rules! or_server_error {
    ($param: expr, $message: tt) => {
        match $param {
            Ok(value) => value,
            Err(err) => {
                warn!("{}: {}", $message, err);

                return Ok(Response::with((status::InternalServerError, $message)));
            }
        }
    }
}

impl Handler for DatabasesHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut body = String::new();

        or_bad_request!(
            request.body.read_to_string(&mut body),
            "Fail to read request"
        );

        let request: DatabasesRequest = or_server_error!(
            serde_json::from_str(&body),
            "Fail to decode request body as JSON"
        );
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

        if request.query.len() > 64 {
            let response = DatabasesResponse::err("Query string too large");
            let json_records = or_server_error!(
                serde_json::to_string(&response),
                "Fail to convert records to JSON"
            );

            Ok(Response::with((content_type, status::Ok, json_records)))
        } else {
            let databases =
                Self::range_databases(request.query.to_lowercase(), self.state.databases());
            let response = DatabasesResponse::ok(databases);
            let json_records = or_server_error!(
                serde_json::to_string(&response),
                "Fail to convert records to JSON"
            );

            Ok(Response::with((content_type, status::Ok, json_records)))
        }
    }
}


pub struct EmptyHandler;


impl EmptyHandler {
    pub fn new() -> EmptyHandler {
        EmptyHandler {}
    }
}


impl Handler for EmptyHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::BadRequest, "No API entry point")))
    }
}


#[derive(Serialize, Debug, Clone)]
struct StateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    ok: bool,
}


pub struct StatusHandler {
    state: State,
}


impl StateResponse {
    fn ok(last_update: Option<i64>) -> StateResponse {
        StateResponse {
            last_update: last_update,
            message: None,
            ok: true,
        }
    }
}


impl StatusHandler {
    pub fn new(state: State) -> StatusHandler {
        StatusHandler { state: state }
    }
}


impl Handler for StatusHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let last_update = self.state.last_update();
        let response = StateResponse::ok(last_update);
        let json_records = or_server_error!(
            serde_json::to_string(&response),
            "Fail to convert records to JSON"
        );
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

        Ok(Response::with((content_type, status::Ok, json_records)))
    }
}
