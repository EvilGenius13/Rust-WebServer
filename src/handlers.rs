use std::convert::Infallible;
use std::fs::File;

use warp::{self, http::StatusCode};
use warp::reply::with_status;

use crate::db::Db;
use crate::models::Pokemon;

pub async fn list_pokemons(db: Db) -> Result<impl warp::Reply, Infallible> {
    let pokemons = db.lock().await;
    let pokemons: Vec<Pokemon> = pokemons.clone();
    Ok(warp::reply::json(&pokemons))
}

pub async fn create_pokemon(
    new_pokemon: Pokemon,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut pokemons = db.lock().await;

    for pokemon in pokemons.iter() {
        if pokemon.id == new_pokemon.id {
            return Ok(with_status(
                "Pokemon with this ID already exists",
                StatusCode::BAD_REQUEST,
            ));
        }
    }

    pokemons.push(new_pokemon.clone());

    // Open the file in write mode
    let file = File::create("./data/pokemon.json").expect("Unable to open file");

    // Write the updated list of Pokemon to the file
    serde_json::to_writer(file, &*pokemons).expect("Unable to write data");

    Ok(with_status(
        "Pokemon created.",
        StatusCode::CREATED,
    ))
}

pub async fn get_pokemon(id: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let pokemons = db.lock().await;

    for pokemon in pokemons.iter() {
        if pokemon.id == id {
            return Ok(Box::new(warp::reply::json(&pokemon)));
        }
    }

    Ok(Box::new(with_status(
        "Pokemon not found.",
        StatusCode::NOT_FOUND,
    )))
}

pub async fn update_pokemon(
    id: String,
    updated_pokemon: Pokemon,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut pokemons = db.lock().await;

    for pokemon in pokemons.iter_mut() {
        if pokemon.id == id {
            *pokemon = updated_pokemon;

            // Open the file in write mode
            let file = File::create("./data/pokemon.json").expect("Unable to open file");

            // Write the updated list of Pokemon to the file
            serde_json::to_writer(file, &*pokemons).expect("Unable to write data");

            return Ok(with_status(
                "Pokemon updated.",
                StatusCode::OK,
            ));
        }
    }

    Ok(with_status(
        "Pokemon not found.",
        StatusCode::NOT_FOUND,
    ))
}

pub async fn delete_pokemon(id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut pokemons = db.lock().await;

    let pokemon_count = pokemons.len();

    pokemons.retain(|pokemon| pokemon.id != id);

    let deleted = pokemons.len() != pokemon_count;
    if deleted {
        // Open the file in write mode
        let file = File::create("./data/pokemon.json").expect("Unable to open file");

        // Write the updated list of Pokemon to the file
        serde_json::to_writer(file, &*pokemons).expect("Unable to write data");

        Ok(with_status(
            "Pokemon deleted.",
            StatusCode::NO_CONTENT,
        ))
    } else {
        Ok(with_status(
            "Pokemon not found.",
            StatusCode::NOT_FOUND,
        ))
    }
}
