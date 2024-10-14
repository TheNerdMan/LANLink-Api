use diesel::prelude::*;
use diesel::sql_types::Integer;
use uuid::Uuid;
use crate::features::equipment::dto::equipment_dto::EquipmentDto;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipmentModel {
    pub id: Integer,
    pub publicid: Uuid,
    pub name: String,
}

impl EquipmentModel {
    pub fn from_dto(dto: &EquipmentDto) -> Result<Self, uuid::Error> {
        Ok(EquipmentModel {
            id: Integer::try_from(dto.id).expect("Huh?"),
            publicid: dto.publicid,
            name: dto.name.clone(),
        })
    }
}
