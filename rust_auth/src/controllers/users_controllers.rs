use crate::models::user_model::Users;
use axum::http::StatusCode;
use axum::{extract, extract::Path, response::IntoResponse, Extension, Json};
use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use bcrypt::{DEFAULT_COST, hash, verify};
use tower_sessions::Session;

use log;


pub async fn users(Extension(_pool): Extension<PgPool>) -> String {
    String::from("users")
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Body {
    name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SubscribeUser {
    username: String,
    mail: String,
    password: String,
}





// const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);


pub async fn test_session(Extension(session): Extension<Session>) -> impl IntoResponse {
    log::info!("{:?}",session);
    session.load().await.unwrap_or_default();
    let counter: Counter = session.get("counter").await.unwrap().unwrap_or_default();
    let name:String =  session.get("username").await.unwrap_or_default().unwrap_or_default();
    
    session.insert("counter", counter.0 + 1).await.unwrap();
    format!("Current count: {} par {} session{:?}", counter.0, name, session)
}


pub async fn logout(Extension(session): Extension<Session>) -> impl IntoResponse {
    if session.is_empty().await {
        (StatusCode::FORBIDDEN, "not connected").into_response()
    }else{
        match session.flush().await{
            Ok(_)=> { (StatusCode::OK, "disconnected").into_response() },
            Err(e)=> {
                eprintln!("error while disconnecting {:?}",e);
                (StatusCode::EXPECTATION_FAILED, "error while disconnecting").into_response()
            }
        }
        
    }
}

pub async fn get_session(Extension(session): Extension<Session>) -> impl IntoResponse {
   
    (StatusCode::ACCEPTED, format!("{:?}", session)).into_response()
    
}


pub async fn login(Extension(session): Extension<Session>)-> impl IntoResponse{
 if session.is_empty().await {
    eprintln!("not connected");
    session.insert("username","kikette".to_string()).await.unwrap_or_default();
    (StatusCode::EXPECTATION_FAILED, "not connected").into_response()
 }else{
    let name:String =  session.get("username").await.unwrap_or_default().unwrap_or_default();
    eprintln!("already connected");
    (StatusCode::ACCEPTED, format!("already connected {}",name)).into_response()
 }
}


pub async fn subscribe(
    Extension(pool): Extension<PgPool>,
    extract::Json(body): extract::Json<SubscribeUser>,
) -> impl IntoResponse {
    let hashed_password = hash(body.password, DEFAULT_COST).unwrap_or("notapass".to_string());
    let info = format!("{} {} {}",body.username, body.mail, hashed_password);
    (StatusCode::INTERNAL_SERVER_ERROR, info).into_response()
}

pub async fn all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Users>("SELECT * FROM Users")
        .fetch_all(&pool)
        .await
    {
        Ok(users) => Json(users).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}



//CRUD BASICS

pub async fn one_user(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Users>("SELECT * FROM Users WHERE id = ?")
        .bind(id)
        .fetch_all(&pool)
        .await
    {
        Ok(users) => Json(users).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn modify_user(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
    extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Users>("UPDATE Users SET name = ? WHERE id = ?")
        .bind(body.name)
        .bind(id)
        .fetch_all(&pool)
        .await
    {
        Ok(_users) => (StatusCode::OK, "user modfied".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Users>("INSERT INTO Users (name) VALUES (?) ")
        .bind(body.name)
        .fetch_all(&pool)
        .await
    {
        Ok(_users) => (StatusCode::OK, "user created".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Users>("DELETE FROM Users  WHERE id = ?")
        .bind(id)
        .fetch_all(&pool)
        .await
    {
        Ok(_users) => (StatusCode::OK, "user deleted".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}
