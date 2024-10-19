use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;

// internal uses
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use diesel::prelude::*;
use crate::schema::gameservers::dsl::*;
use crate::features::game_server::models::game_server_model::GameServerModel;
use crate::schema::gameservers::dsl::gameservers;
use crate::schema::gameservers::{game_type, game_server_title};

pub async fn get_all_game_servers(
    pool: &Pool,
) -> Option<Vec<GameServerModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        gameservers
            .select(GameServerModel::as_select())
            .load::<GameServerModel>(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_game_server_by_id(
    pool: &Pool,
    request_id: i32,
) -> Option<GameServerModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        gameservers::table()
            .find(request_id)
            .select(GameServerModel::as_select())
            .first(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}



pub async fn get_game_server_by_public_id(
    pool: &Pool,
    public_id: Uuid,
) -> Option<GameServerModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        gameservers::table()
            .filter(publicid.eq(public_id))
            .select(GameServerModel::as_select())
            .first(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_game_server_by_name(
    pool: &Pool,
    req_server_name: String,
) -> Option<GameServerModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        gameservers::table()
            .filter(game_server_title.eq(req_server_name))
            .select(GameServerModel::as_select())
            .first(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_all_game_servers_by_game_type(
    pool: &Pool,
    req_game_type: String,
) -> Option<Vec<GameServerModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        gameservers
            .select(GameServerModel::as_select())
            .filter(game_type.eq(req_game_type))
            .load::<GameServerModel>(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}
pub async fn create_or_update_game_server(
    pool: &Pool,
    game_server_model: GameServerModel,
) -> Option<GameServerModel> {
    if game_server_model.id == 0 {
        return create_game_server(pool, game_server_model).await
    } else {
        return update_game_server(pool, game_server_model).await
    }
}

async fn create_game_server(
    pool: &Pool,
    game_server_model: GameServerModel,
) -> Option<GameServerModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        diesel::insert_into(gameservers::table())
            .values(game_server_model.create_new_game_server_for_db())
            .returning(GameServerModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

async fn update_game_server(
    pool: &Pool,
    game_server_model: GameServerModel,
) -> Option<GameServerModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        diesel::update(gameservers::table())
            .filter(id.eq(game_server_model.id))
            .set(game_server_model.create_update_game_server_for_db())
            .returning(GameServerModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(server) => {
            match server {
                Ok(server) => Some(server),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn drop_game_server(
    pool: &Pool,
    request_id: i32,
) -> Result<usize, AppError> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return Err(AppError::DatabaseQueryError(format!("Unable to establish database connection"))),
    };

    let result = conn.interact(move |c| {
        // Here, execute returns a QueryResult<usize>
        diesel::delete(gameservers::table()
            .filter(id.eq(request_id))) // Ensure you're referencing gameservers::id
            .execute(c) // Execute the delete operation
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()))?;

    Ok(result.unwrap())
}
