pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241202_113447_create_table;
mod m20241203_120431_create_table;
mod m20241210_120723_create_table;
mod m20241210_125348_insert_data;
mod m20241231_133646_create_table;
mod m20250108_084238_create_table_path_info;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241202_113447_create_table::Migration),
            Box::new(m20241203_120431_create_table::Migration),
            Box::new(m20241210_120723_create_table::Migration),
            Box::new(m20241210_125348_insert_data::Migration),
            Box::new(m20241231_133646_create_table::Migration),
            Box::new(m20250108_084238_create_table_path_info::Migration),
        ]
    }
}
