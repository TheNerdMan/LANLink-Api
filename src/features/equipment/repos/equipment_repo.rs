use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;


// internal uses
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use crate::core::errors::error_handler::throw_error;
use crate::features::equipment::models::equipment_model::EquipmentModel;
use crate::schema::equipments::dsl::*;

pub async fn get_all_equipment(
    pool: Pool,
) -> Option<Vec<EquipmentModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        equipments::table()
            .select(EquipmentModel::as_select())
            .expect("Error getting all equipment")
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => Some(equipment),
        Err(e) => {
            throw_error(e);
            None
        }
    }
}

pub async fn get_equipment_by_id(
    pool: Pool,
    id: i32,
) -> Option<EquipmentModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        equipments::table()
            .find(id)
            .select(EquipmentModel::as_select())
            .expect("Error getting equipment by id")
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => Some(equipment),
        Err(_) => None
    }
}

pub async fn get_equipment_by_public_id(
    pool: Pool,
    public_id: Uuid,
) -> Option<EquipmentModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        equipments::table()
            .filter(publicid.eq(public_id))
            .select(EquipmentModel::as_select())
            .expect("Error getting equipment by public id")
    })
    .await
    .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => Some(equipment),
        Err(_) => None
    }
}

