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
