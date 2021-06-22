use hyper::{Body, Request, Response, StatusCode};
use route_recognizer::Router;

type ResultResponseHyper = Result<Response<Body>, hyper::Error>;
type RequestHyper = Request<Body>;

mod category;

pub async fn routes(req: RequestHyper) -> ResultResponseHyper {
    let mut router = Router::new();
    router.add("/category/*action", category::configure());
    router.add("/category", category::configure());

    match router.recognize(req.uri().path()) {
        Ok(handle) => {
            handle.handler().call(req, handle.params())
        },
        _ => get_response_by_status_code(StatusCode::NOT_FOUND)
    }
}

fn get_response_by_status_code(status_code: StatusCode) -> ResultResponseHyper {
    let mut response = Response::default();
    *response.status_mut() = status_code;
    Ok(response)
}

async fn parse_body(req: RequestHyper) -> String {
    String::from_utf8(
        hyper::body::to_bytes(req).await.unwrap().to_vec()
    ).unwrap()
}
