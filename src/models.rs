use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Pokemon {
    pub id: String,
    pub name: String,
    pub poke_type: String,
}
