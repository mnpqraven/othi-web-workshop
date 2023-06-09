use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("REQUEST ", body, None).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    // otherside of the middleware after running code, repeat for response logs
    let res = next.run(req).await;
    let status_code = Some(res.status());
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("RESPONSE", body, status_code).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(
    direction: &str,
    body: B,
    status_code: Option<StatusCode>,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    let status_code = match status_code {
        Some(code) => format!("{:?} ", code),
        None => format!("None"),
    };
    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!(
            "{} STATUS = {} BODY = {:?} ",
            direction,
            status_code,
            body
        );
    }

    Ok(bytes)
}
