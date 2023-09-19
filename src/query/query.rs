use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Is<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Serialize, Deserialize)]
pub enum Data<T> {
    Item(T),
    Location(T),
}

