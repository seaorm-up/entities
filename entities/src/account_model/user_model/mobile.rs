use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "profile")]
#[graphql(complex)]
#[graphql(name = "Profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    // pub id: u64,
    pub mobile: u64,
    pub account_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
