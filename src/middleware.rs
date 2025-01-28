use crate::{auth, AppState};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::middleware::Next;
use actix_web::{web, HttpMessage};

fn get_header(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|token| {
            if token.starts_with("Bearer ") {
                Some(token[7..].to_string())
            } else {
                None
            }
        })
}

pub async fn refresh_token_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = match get_header(&req) {
        Some(token) => token,
        None => return Err(actix_web::error::ErrorUnauthorized("")),
    };

    let app_data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError(""))?;

    let claims = auth::jwt::RefreshTokenClaims::decode(&token, &app_data.jwt_secret)
        .map_err(|_| actix_web::error::ErrorUnauthorized(""))?;

    req.extensions_mut().insert(claims);
    next.call(req).await
}

pub async fn access_token_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = match get_header(&req) {
        Some(token) => token,
        None => return Err(actix_web::error::ErrorUnauthorized("")),
    };

    let app_data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError(""))?;

    let claims = auth::jwt::AccessTokenClaims::decode(&token, &app_data.jwt_secret)
        .map_err(|_| actix_web::error::ErrorUnauthorized(""))?;
    req.extensions_mut().insert(claims);
    next.call(req).await
}
