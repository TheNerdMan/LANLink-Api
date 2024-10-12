use diesel::prelude::*;
use diesel::sql_types::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipmentModel {
    pub id: u32,
    pub public_id: Uuid,
    pub name: String,
}