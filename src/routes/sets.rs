use crate::routes::responses::sets::SetListResponse;
use actix_web::{get, HttpResponse};

#[get("/sets")]
pub async fn get_sets_list() -> HttpResponse {
    HttpResponse::Ok().json(SetListResponse::new(vec![]))
}
