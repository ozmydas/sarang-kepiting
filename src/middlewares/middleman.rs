use axum::{extract::Request, middleware::Next, response::Response};

pub async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...
    println!("🔥 req : {:#?}", request);

    let response = next.run(request).await;

    // do something with `response`...

    response
}