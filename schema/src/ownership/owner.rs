use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "owner")]
#[graphql(complex)]
#[graphql(name = "Owner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub obj_id: u64,
    #[sea_orm(primary_key)]
    pub account_id: u64,
    #[sea_orm(primary_key)]
    pub owner_kind: OwnerKind,
    pub invited_by_account: u64,
}
#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "TinyInteger")]
pub enum OwnerKind {
    User = 0,
    Team = 1,
}
