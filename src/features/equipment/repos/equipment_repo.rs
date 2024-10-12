use diesel::associations::HasTable;
use diesel::prelude::*;
use uuid::Uuid;

// internal uses
use crate::core::pg_connector::establish_connection;
use crate::features::equipment::models::equipment_model::EquipmentModel;

pub fn get_all_equipment() -> Option<Vec<EquipmentModel>> {
    use crate::schema::equipments::dsl::*;

    let conn = &mut establish_connection();

    let result = equipments
                    .select(equipments::as_select())
                    .load(conn)
                    .optional();
    match result {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(_) => panic!("Couldn't get equipments from database"),
    }
}

pub fn get_equipment(public_id: Uuid) -> Option<EquipmentModel> {
    use crate::schema::equipments::dsl::equipments;

    let conn = &mut establish_connection();


    let result = equipments
        .find(public_id)
        .select(EquipmentModel::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(_) => panic!("Couldn't get equipments from database"),
    }
}

pub fn create_equipment(equipment: &EquipmentModel) -> EquipmentModel {
    use crate::schema::equipments::dsl::equipments;

    let conn = &mut establish_connection();

    diesel::insert_into(equipments::table)
        .values(&equipment)
        .returning(EquipmentModel::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment")
}