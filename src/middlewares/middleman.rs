use std::env;

use argon2::{password_hash, Argon2, PasswordVerifier};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use serde_json::Value;

use crate::utils::{
    custom_response::{output_json, ApiError, ResponseResult},
    date_jakarta,
};

/// referensi : https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html
pub async fn my_middleware(
    request: Request,
    next: Next,
) -> Result<Response, ResponseResult<Json<Value>>> {
    // do something with `request`...
    let headers = request.headers();

    let token = headers
        .get("z-token")
        .map_or("jancok", |val| val.to_str().unwrap());

    let date = headers
        .get("z-date")
        .map_or("2000-01-01", |val| val.to_str().unwrap());

    let uid = headers
        .get("z-uuid")
        .map_or("0000", |val| val.to_str().unwrap());

    // verify headders
    let mut message = "UNAUTHORIZED".to_string();
    let parsed = password_hash::PasswordHash::new(token);

    let allowed = if let Ok(parsedhash) = parsed {
        let password = format!("{}.{}.{}", env::var("APP_KEY").unwrap(), uid, date);

        match Argon2::default().verify_password(password.as_bytes(), &parsedhash) {
            Ok(_) => {
                let differ =
                    date_jakarta::date_diff(date_jakarta::today_jakarta(), date.to_string());

                if differ < 0 || differ > 3 {
                    message = "UNAUTHORIZED: Auth Token Timeout!".to_string();
                    Err(false)
                } else {
                    Ok(true)
                }
            }
            Err(_) => {
                message = "UNAUTHORIZED: Auth Token Miss-Match!".to_string();
                Err(false)
            }
        }
    } else {
        message = "UNAUTHORIZED: Invalid Token Format!".to_string();
        Err(false)
    };

    // do something with `response`...
    let response = next.run(request).await;

    match allowed {
        Ok(_) => Ok(response),
        Err(_) => Err(output_json(false, message, [0; 0], Some(401))),
    }
}
