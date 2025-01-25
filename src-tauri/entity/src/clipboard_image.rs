//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "clipboard_image"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub id: Uuid,
    pub clipboard_id: Uuid,
    pub data: Vec<u8>,
    pub thumbnail: String,
    pub extension: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub size: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    ClipboardId,
    Data,
    Thumbnail,
    Extension,
    Width,
    Height,
    Size,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Clipboard,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Uuid.def(),
            Self::ClipboardId => ColumnType::Uuid.def().unique(),
            Self::Data => ColumnType::Blob.def(),
            Self::Thumbnail => ColumnType::Text.def(),
            Self::Extension => ColumnType::String(StringLen::None).def().null(),
            Self::Width => ColumnType::Integer.def().null(),
            Self::Height => ColumnType::Integer.def().null(),
            Self::Size => ColumnType::String(StringLen::None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Clipboard => Entity::belongs_to(super::clipboard::Entity)
                .from(Column::ClipboardId)
                .to(super::clipboard::Column::Id)
                .into(),
        }
    }
}

impl Related<super::clipboard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clipboard.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
