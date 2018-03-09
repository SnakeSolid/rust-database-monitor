use std::cmp::Ordering;
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

use search::Query;
use state::DatabaseRow;
use state::State;

#[derive(Deserialize, Debug, Clone)]
struct DatabasesRequest {
    query: String,
}

#[derive(Serialize, Debug, Clone)]
struct Database {
    server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")] server_description: Option<String>,
    database_name: String,
    collation_name: String,
    #[serde(skip_serializing_if = "Option::is_none")] commit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] project_name: Option<String>,
    role_name: String,
    last_update: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
struct DatabasesResponse {
    #[serde(skip_serializing_if = "Option::is_none")] databases: Option<Vec<Database>>,
    #[serde(skip_serializing_if = "Option::is_none")] message: Option<String>,
    ok: bool,
}

pub struct DatabasesHandler {
    state: State,
}

impl DatabasesResponse {
    fn ok(databases: Vec<Database>) -> DatabasesResponse {
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

impl Database {
    fn new(
        server_name: &str,
        server_description: &Option<String>,
        database_name: &str,
        collation_name: &str,
        commit: &Option<i64>,
        branch_name: &Option<String>,
        project_name: &Option<String>,
        role_name: &str,
        last_update: Option<i64>,
    ) -> Database {
        Database {
            server_name: server_name.into(),
            server_description: server_description.clone(),
            database_name: database_name.into(),
            collation_name: collation_name.into(),
            commit: commit.clone(),
            branch_name: branch_name.clone(),
            project_name: project_name.clone(),
            role_name: role_name.into(),
            last_update,
        }
    }
}

impl DatabasesHandler {
    pub fn new(state: State) -> DatabasesHandler {
        DatabasesHandler { state: state }
    }

    fn query_databases(&self, query: &Query) -> Vec<Database> {
        let mut databases = self.state.query(query);
        databases.sort_by(Self::compare_databases);

        databases
            .into_iter()
            .take(30)
            .map(|database| {
                Database::new(
                    database.server_name(),
                    database.server_description(),
                    database.database_name(),
                    database.database_collate(),
                    database.commit(),
                    database.branch_name(),
                    database.project_name(),
                    database.database_owner(),
                    Some(database.last_update()),
                )
            })
            .collect()
    }

    fn compare_databases(a: &DatabaseRow, b: &DatabaseRow) -> Ordering {
        let a_weight = a.weight();
        let b_weight = b.weight();

        if a_weight < b_weight {
            return Ordering::Greater;
        } else if a_weight > b_weight {
            return Ordering::Less;
        } else if a.database_name() < b.database_name() {
            return Ordering::Greater;
        } else if a.database_name() > b.database_name() {
            return Ordering::Less;
        } else if a.server_name() < b.server_name() {
            return Ordering::Greater;
        } else if a.database_name() > b.database_name() {
            return Ordering::Less;
        } else {
            Ordering::Equal
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
            let query = request.query.into();
            let databases = self.query_databases(&query);
            let response = DatabasesResponse::ok(databases);
            let json_records = or_server_error!(
                serde_json::to_string(&response),
                "Fail to convert records to JSON"
            );

            Ok(Response::with((content_type, status::Ok, json_records)))
        }
    }
}
