use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Pokemon;

/// All pokemon routes
pub fn pokemon_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_pokemon(db.clone())
        .or(update_pokemon(db.clone()))
        .or(delete_pokemon(db.clone()))
        .or(create_pokemon(db.clone()))
        .or(pokemons_list(db))
}

/// GET /pokemons
fn pokemons_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pokemons")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_pokemons)
}

/// POST /pokemons
fn create_pokemon(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pokemons")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_pokemon)
}

/// GET /pokemons/{id}
fn get_pokemon(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("pokemons" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_pokemon)
}

/// PUT /pokemons/{id}
fn update_pokemon(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("pokemons" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_pokemon)
}

/// DELETE /pokemons/{id}
fn delete_pokemon(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("pokemons" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_pokemon)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Pokemon,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
