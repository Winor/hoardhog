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

// pub type Return<T> = Result<Option<Data<Is<T>>>, sea_orm::DbErr>;
pub type Return<T> = Result<Option<T>, sea_orm::DbErr>;