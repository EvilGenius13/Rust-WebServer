use warp;

mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let pokemon_routes = routes::pokemon_routes(db);

    warp::serve(pokemon_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
