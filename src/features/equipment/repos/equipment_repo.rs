use deadpool_diesel::postgres::Pool;
use diesel::associations::HasTable;
use uuid::Uuid;
use crate::core::db_connection::db_connection::create_connection;


// internal uses
use crate::core::errors::error::AppError;
use crate::core::errors::error::AppError::DatabaseQueryError;
use crate::core::errors::error_handler::throw_error;
use crate::features::equipment::models::equipment_model::EquipmentModel;
use crate::schema::equipments::dsl::equipments;

pub async fn get_all_equipment(
    pool: Pool,
) -> Option<Vec<EquipmentModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|conn| {
        equipments::table
            .load(conn)
    })
    .await
    .map_err(|e| DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => Some(equipment),
        Err(e) => {
            throw_error(e);
            None
        }
    }
}

pub async fn get_equipment(
    pool: Pool,
    publicid: Uuid,
) -> Option<EquipmentModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|conn| {
        equipments::table
            .find(publicid)   // Search by the equipment_id
            .first::<EquipmentModel>(conn) // Get the first match (or error)
    })
    .await
    .map_err(|e| DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => Some(equipment),
        Err(_) => None
    }
}

