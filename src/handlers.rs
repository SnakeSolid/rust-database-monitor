use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;

use rustc_serialize::json;

use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::Handler;
use iron::status;
use iron::mime::Mime;
use iron::mime::TopLevel;
use iron::mime::SubLevel;

use state::State;
use state::DatabaseRow;


#[derive(RustcDecodable, Debug, Clone)]
struct DatabasesRequest {
    query: String,
}


#[derive(RustcEncodable, Debug, Clone)]
struct DatabaseInfo {
    database_name: String,
    collation_name: String,
    role_name: String,
    server_name: String,
    last_update: Option<i64>,
}


#[derive(RustcEncodable, Debug, Clone)]
struct DatabasesResponse {
    databases: Option<Vec<DatabaseInfo>>,
    message: Option<String>,
    ok: bool,
}


pub struct DatabasesHandler {
    state: State,
}


impl DatabasesHandler {
    pub fn new(state: State) -> DatabasesHandler {
        DatabasesHandler { state: state }
    }

    fn range_databases<S>(query: S, database_list: Vec<DatabaseRow>) -> Vec<DatabaseInfo>
        where S: Into<String>
    {
        let query = query.into();
        let mut ranks = HashMap::new();

        for part in query.split(' ').filter(|e| !e.is_empty()) {
            for database in &database_list {
                let database_name = database.database_name.to_uppercase();

                if database_name.contains(part) {
                    let count = ranks.entry(database).or_insert(0);

                    *count += part.len();
                }
            }
        }

        let mut ranks: Vec<_> = ranks.into_iter().collect();

        ranks.sort_by(Self::compare_databases);

        let result: Vec<_> = ranks.into_iter()
            .take(20)
            .map(|(db, _)| {
                DatabaseInfo {
                    database_name: db.database_name.clone(),
                    collation_name: db.database_collate.clone(),
                    role_name: db.database_owner.clone(),
                    server_name: db.server_name.clone(),
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


impl Handler for DatabasesHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut body = String::new();

        if let Err(err) = request.body.read_to_string(&mut body) {
            warn!("Fail to read request: {}", err);

            return Ok(Response::with((status::BadRequest, "Fail to read request")));
        }

        let request: DatabasesRequest = match json::decode(&body) {
            Ok(request) => request,
            Err(err) => {
                warn!("Fail to decode request body as JSON: {}", err);

                return Ok(Response::with((status::BadRequest,
                                          "Fail to decode request body as JSON")));
            }
        };

        if request.query.len() > 32 {
            info!("Query string too large");

            return Ok(Response::with((status::BadRequest, "Query string too large")));
        }

        let databases = Self::range_databases(request.query.to_uppercase(), self.state.databases());

        let response = DatabasesResponse {
            databases: Some(databases),
            message: None,
            ok: true,
        };

        let json_records = match json::encode(&response) {
            Ok(json_records) => json_records,
            Err(err) => {
                warn!("Fail to convert records to JSON: {}", err);

                return Ok(Response::with((status::InternalServerError,
                                          "Fail to convert records to JSON")));
            }
        };

        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

        Ok(Response::with((content_type, status::Ok, json_records)))
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


#[derive(RustcEncodable, Debug, Clone)]
struct StateResponse {
    last_update: Option<i64>,
    message: Option<String>,
    ok: bool,
}


pub struct StatusHandler {
    state: State,
}


impl StatusHandler {
    pub fn new(state: State) -> StatusHandler {
        StatusHandler { state: state }
    }
}


impl Handler for StatusHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let last_update = self.state.last_update();

        let response = StateResponse {
            last_update: last_update,
            message: None,
            ok: true,
        };

        let json_records = match json::encode(&response) {
            Ok(json_records) => json_records,
            Err(err) => {
                warn!("Fail to convert records to JSON: {}", err);

                return Ok(Response::with((status::InternalServerError,
                                          "Fail to convert records to JSON")));
            }
        };

        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

        Ok(Response::with((content_type, status::Ok, json_records)))
    }
}
