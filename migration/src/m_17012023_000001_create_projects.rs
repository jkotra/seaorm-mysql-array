use super::m_17012023_000001_create_employee::Employee;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .col(
                        ColumnDef::new(Projects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::EmpId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-emp-projects")
                            .from(Projects::Table, Projects::EmpId)
                            .to(Employee::Table, Employee::Id),
                    )
                    .col(ColumnDef::new(Projects::Seq).integer().not_null())
                    .col(ColumnDef::new(Projects::Value).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Projects {
    Table,
    Id,
    EmpId,
    Seq,
    Value,
}
