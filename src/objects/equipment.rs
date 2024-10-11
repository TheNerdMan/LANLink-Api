use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Equipment {
    pub id: u32,
    pub public_id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct HireEquipment {
    pub id: u32,
    pub public_id: Uuid,
    pub equipment: Equipment
}
