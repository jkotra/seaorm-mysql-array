//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub emp_id: i32,
    pub seq: i32,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmpId",
        to = "super::employee::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Employee,
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
