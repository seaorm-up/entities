use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "tenant")]
#[graphql(complex)]
#[graphql(name = "Tenant")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    /// TenantKey
    pub id: TenantFlowId,
    #[filter(use_alias = true)]
    pub tenant_key: TenantKey,
    #[filter(use_alias = true)]
    pub app_token: AppAccessToken,
    // pub appIid: AppId,
    /// secret key to dev
    /// Permissions on data
    #[filter(use_alias = true)]
    pub scope: AppPermissions,
    /// token for device app center
    #[filter(use_alias = true)]
    pub tenant_token: TenantAccessToken,
    pub tenant_name: String,
    pub redirect_uri: String,
    /// The callback address of the event subscription for [app_ticket]
    pub event_callback_address: String,
    #[filter(use_alias = true)]
    pub expire: TimeAlias,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
// impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
