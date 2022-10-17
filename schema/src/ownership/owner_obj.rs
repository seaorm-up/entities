use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "obj_owner")]
#[graphql(complex)]
#[graphql(name = "ObjOwner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub obj_id: u32,
    #[sea_orm(primary_key)]
    pub owner_id: u32,
}
