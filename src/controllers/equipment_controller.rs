use axum::{extract::Path, response::Json, routing::post, Router};
use axum::http::StatusCode;
use uuid::{Uuid};

use crate::objects::equipment::{Equipment, HireEquipment};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/equipment/hire/:id", post(hire))
        .route("/api/v1/equipment/hire-by-name/:equipmentName", post(hire_by_name))
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
    (StatusCode::FOUND, Json(rsp))
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
    (StatusCode::FOUND, Json(rsp))
}