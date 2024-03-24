pub use sea_orm_migration::prelude::*;

pub mod m20240322_122454_create_user_table;
pub mod m20240324_173148_create_stores_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240322_122454_create_user_table::Migration),
            Box::new(m20240324_173148_create_stores_table::Migration),
        ]
    }
}
