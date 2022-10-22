use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "open")]
#[graphql(complex)]
#[graphql(name = "Open")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: OpenId,
    #[filter(use_alias = true)]
    pub app_id: AccountId,
    pub created_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
// impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
