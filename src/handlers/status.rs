use serde_json;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::mime::SubLevel;
use iron::mime::TopLevel;
use iron::Request;
use iron::Response;
use iron::status;

use state::State;

#[derive(Serialize, Debug, Clone)]
struct StatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")] last_update: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] message: Option<String>,
    ok: bool,
}

pub struct StatusHandler {
    state: State,
}

impl StatusResponse {
    fn ok(last_update: Option<i64>) -> StatusResponse {
        StatusResponse {
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
        let response = StatusResponse::ok(last_update);
        let json_records = or_server_error!(
            serde_json::to_string(&response),
            "Fail to convert records to JSON"
        );
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

        Ok(Response::with((content_type, status::Ok, json_records)))
    }
}
