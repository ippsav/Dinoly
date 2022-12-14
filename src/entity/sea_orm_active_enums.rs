//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "provider")]
pub enum Provider {
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "local")]
    Local,
}
