//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "clipboard"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub id: Uuid,
    pub types: Json,
    pub star: bool,
    pub encrypted: bool,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Types,
    Star,
    Encrypted,
    CreatedAt,
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
    ClipboardFile,
    ClipboardHtml,
    ClipboardImage,
    ClipboardRtf,
    ClipboardText,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Uuid.def(),
            Self::Types => ColumnType::Json.def(),
            Self::Star => ColumnType::Boolean.def(),
            Self::Encrypted => ColumnType::Boolean.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ClipboardFile => Entity::has_many(super::clipboard_file::Entity).into(),
            Self::ClipboardHtml => Entity::has_one(super::clipboard_html::Entity).into(),
            Self::ClipboardImage => Entity::has_one(super::clipboard_image::Entity).into(),
            Self::ClipboardRtf => Entity::has_one(super::clipboard_rtf::Entity).into(),
            Self::ClipboardText => Entity::has_one(super::clipboard_text::Entity).into(),
        }
    }
}

impl Related<super::clipboard_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClipboardFile.def()
    }
}

impl Related<super::clipboard_html::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClipboardHtml.def()
    }
}

impl Related<super::clipboard_image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClipboardImage.def()
    }
}

impl Related<super::clipboard_rtf::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClipboardRtf.def()
    }
}

impl Related<super::clipboard_text::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClipboardText.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
