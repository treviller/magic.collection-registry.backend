use actix_web::{get, web, HttpResponse};

use crate::domain::set::SetService;
use crate::dto::set::SetDto;
use crate::provider::database::DbConnection;
use crate::routes::responses::sets::SetListResponse;

#[get("/sets")]
pub async fn get_sets_list(db_pool: web::Data<DbConnection>) -> HttpResponse {
    let set_service = SetService::new(&db_pool);

    let sets = set_service.get_sets_list().await.unwrap();

    let set_dtos = sets.into_iter().map(|set| SetDto::new(set)).collect();

    HttpResponse::Ok().json(SetListResponse::new(set_dtos))
}
