use crate::*;

/// [app_ticket] in ticket would be used.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "app")]
#[graphql(complex)]
#[graphql(name = "App")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AppId,
    /// secret key to dev
    pub app_secret: String,
    /// Permissions on data
    #[filter(use_alias = true)]
    pub scope: AppPermissions,
    #[filter(use_alias = true)]
    pub app_token: AppAccessToken,
    pub app_name: String,
    #[filter(use_alias = true)]
    pub expire: TimeAlias,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
// impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
