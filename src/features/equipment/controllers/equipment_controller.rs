use axum::{extract::Path, response::Json, routing::post, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Serialize;
use uuid::{Uuid};
use crate::features::equipment::mapper::equipment_mapper::model_to_dto;
use crate::features::equipment::models::equipment_model::EquipmentModel;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/equipment", get(get_all_equipment_handler))
        .route("/api/v1/equipment/:public_id", get(get_equipment_handler))
        .route("/api/v1/equipment/hire/:id", post(hire))
        .route("/api/v1/equipment/hire-by-name/:equipmentName", post(hire_by_name))
}

#[axum::debug_handler]
async fn get_all_equipment_handler() -> impl IntoResponse {
    use crate::features::equipment::repos::equipment_repo::get_all_equipment;

    let equipment_models = get_all_equipment();

    match equipment_models {
        Some(vec) => {
            let dtos = vec.iter().map(|e| model_to_dto(e));
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}

#[axum::debug_handler]
async fn get_equipment_handler(Path(public_id): Path<Uuid>) -> impl IntoResponse {
    use crate::features::equipment::repos::equipment_repo::get_equipment;

    let equipment = get_equipment(public_id);

    match equipment {
        Some(item) => {
            let dtos = model_to_dto(&item);
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}

#[derive(Serialize)]
struct Equipment {
    pub id: u32,
    pub public_id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
struct HireEquipment {
    pub id: u32,
    pub public_id: Uuid,
    pub equipment: Equipment
}

async fn hire(Path(id): Path<u32>) -> (StatusCode, Json<HireEquipment>) {
    let rsp = HireEquipment {
        id: 1,
        public_id: Uuid::new_v4(),
        equipment: Equipment{
            id,
            public_id: Uuid::new_v4(),
            name: "Big wire".parse().unwrap()
        }
    };
    (StatusCode::OK, Json(rsp))
}

async fn hire_by_name(Path(equipment_name): Path<String>) -> (StatusCode, Json<HireEquipment>) {
    let rsp = HireEquipment {
        id: 2,
        public_id: Uuid::new_v4(),
        equipment: Equipment{
            id: 3,
            public_id: Uuid::new_v4(),
            name: equipment_name
        }
    };
    (StatusCode::OK, Json(rsp))
}
