use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;

// internal uses
use crate::core::db_connection::db_connection::create_connection;
use crate::core::errors::error::AppError;
use crate::features::user::models::user_model::UserModel;
use crate::schema::users::dsl::*;

pub async fn get_all_users(
    pool: Pool,
) -> Option<Vec<UserModel>> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(|c| {
        users
            .select(UserModel::as_select())
            .load::<UserModel>(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_user_by_id(
    pool: Pool,
    request_id: i32,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .find(request_id)
            .select(UserModel::as_select())
            .first(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn get_user_by_public_id(
    pool: Pool,
    public_id: Uuid,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    let result = conn.interact(move |c| {
        users::table()
            .filter(publicid.eq(public_id))
            .select(UserModel::as_select())
            .first(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub async fn create_or_update_user(
    pool: Pool,
    user_model: UserModel,
)  {
    if user_model.id == 0 {
        create_user(pool, user_model).await;
    } else {
        update_user(pool, user_model).await;
    }
}

async fn create_user(
    pool: Pool,
    user_model: UserModel,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    // This should be abstracted out to the UserModel impl
    let mut new_user = UserModel::new();
    new_user.publicid = Uuid::new_v4();
    new_user.first_name = user_model.first_name;
    new_user.last_name = user_model.last_name;
    new_user.discord_username = user_model.discord_username;
    new_user.steam_url = user_model.steam_url;
    
    let result = conn.interact(move |c| {
        diesel::insert_into(users::table())
            .values(new_user)
            .returning(UserModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

async fn update_user(
    pool: Pool,
    user_model: UserModel,
) -> Option<UserModel> {
    let conn = match create_connection(pool).await {
        Some(conn) => conn,
        None => return None,
    };

    // This should be abstracted out to the UserModel impl
    // we are making sure not to delete the user public ID as it should be static
    let mut update_user = UserModel::new();
    update_user.id = user_model.id;
    update_user.first_name = user_model.first_name;
    update_user.last_name = user_model.last_name;
    update_user.discord_username = user_model.discord_username;
    update_user.steam_url = user_model.steam_url;
    
    let result = conn.interact(move |c| {
        diesel::update(users::table())
            .filter(id.eq(update_user.id))
            .set(&update_user)
            .returning(UserModel::as_returning())
            .get_result(c)
    })
        .await
        .map_err(|e| AppError::DatabaseQueryError(e.to_string()));

    match result {
        Ok(equipment) => {
            match equipment {
                Ok(equipment) => Some(equipment),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}
