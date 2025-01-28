use crate::{auth, AppState};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{web, HttpMessage};

pub async fn refresh_token_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match header {
        Some(token) => {
            if token.starts_with("Bearer ") {
                token[7..].to_string()
            } else {
                return Err(actix_web::error::ErrorUnauthorized(""));
            }
        }
        None => {
            return Err(actix_web::error::ErrorUnauthorized(""));
        }
    };

    let app_data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError(""))?;

    let claims =
        auth::jwt::RefreshTokenClaims::decode(&token, &app_data.jwt_secret).map_err(|_| {
            actix_web::error::ErrorUnauthorized("")
        })?;

    req.extensions_mut().insert(claims);
    next.call(req).await
}
