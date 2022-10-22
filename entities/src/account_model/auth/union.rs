use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "union")]
#[graphql(complex)]
#[graphql(name = "Union")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: AccountId,
    pub app_id: u32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
