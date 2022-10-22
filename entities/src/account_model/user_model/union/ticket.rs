use crate::*;

/// get token for device app center, refresh with time(such as 2 hours)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "ticket")]
#[graphql(complex)]
#[graphql(name = "Ticket")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AppId,
    /// maybe we can include machine info.
    /// gen with id + app_secret
    #[filter(use_alias = true)]
    pub app_ticket: TicketValue,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
// impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
