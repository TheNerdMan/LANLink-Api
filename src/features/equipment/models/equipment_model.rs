use diesel::prelude::*;
use diesel::sql_types::{Integer, Uuid as DieselUuid};
use crate::features::equipment::dto::equipment_dto::EquipmentDto;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipmentModel {
    pub id: Integer,
    pub publicid: DieselUuid,
    pub name: String,
}

impl EquipmentModel {
    pub fn from_dto(dto: &EquipmentDto) -> Result<Self, uuid::Error> {
        Ok(EquipmentModel {
            id: dto.id.into(),
            publicid: dto.publicid.into(),
            name: dto.name.clone(),
        })
    }
}
