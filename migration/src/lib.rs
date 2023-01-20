pub use sea_orm_migration::prelude::*;

mod m_17012023_000001_create_employee;
mod m_17012023_000001_create_projects;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_17012023_000001_create_employee::Migration),
            Box::new(m_17012023_000001_create_projects::Migration),
        ]
    }
}
