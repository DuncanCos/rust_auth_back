use sqlx::postgres::PgPoolOptions;
mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/db_name").await.expect("error whit db connection");


    println!("Connected to MySQL");

    // build our application with a single route
    let app = routes::routing(pool);

    // run our app with hyper, listening globally on port 8000
    println!("server launched on http://127.0.0.1:8080");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}