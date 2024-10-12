use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct EquipmentDto {
    pub id: u32,
    pub public_id: Uuid,
    pub name: String,
}