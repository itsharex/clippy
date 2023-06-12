//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "hotkey"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub event: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
    pub status: bool,
    pub name: String,
    pub icon: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Event,
    Ctrl,
    Alt,
    Shift,
    Key,
    Status,
    Name,
    Icon,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Event => ColumnType::String(None).def(),
            Self::Ctrl => ColumnType::Boolean.def(),
            Self::Alt => ColumnType::Boolean.def(),
            Self::Shift => ColumnType::Boolean.def(),
            Self::Key => ColumnType::String(None).def(),
            Self::Status => ColumnType::Boolean.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Icon => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
