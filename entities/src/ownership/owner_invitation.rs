use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "obj_owner_jour")]
#[graphql(complex)]
#[graphql(name = "ObjOwnerJour")]
pub struct Model {
    pub obj_id: u64,
    pub invited_account: u64,
    pub invited_by_account: u64,
    pub created_at: NaiveDateTime,
    pub token: String,
    pub token_created_at: Option<NaiveDateTime>,
}
