pub use sea_orm_migration::prelude::*;

pub mod m20221121_170216_create_user_table;
pub mod m20221213_173521_create_url_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221121_170216_create_user_table::Migration),
            Box::new(m20221213_173521_create_url_table::Migration),
        ]
    }
}
