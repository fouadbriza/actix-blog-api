use actix_web::{Responder, get, web};

use crate::utils::{api_response, app_state::AppState};

#[get("")]
pub async fn user(app_state: web::Data<AppState>) -> impl Responder {
    
    api_response::ApiResponse::new(200,String::from("validated"))
    
}
