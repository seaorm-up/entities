use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "union")]
#[graphql(complex)]
#[graphql(name = "Union")]
/// source from https://www.apifox.cn/web/project/532425
/// 创建的应用不存在跨应用的数据关联互通场景，使用 open_id 即可；
/// 需要跨应用的数据关联，但是其开发者归属同一个企业组织，可以使用 union_id；
/// 需要跨应用关联，并且这些应用可能是不同的组织开发的，但是应用的用户归属在同一个企业内，则使用user_id；需要注意的是 user_id只允许自建应用申请，需要申请 获得用户userid 这个额外的权限
/// OpenId=UnionId+Time, UnionId=AccountId+AppId
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: OpenId,
    pub created_at: DateTimeUtc,
    #[filter(use_alias = true)]
    pub union_id: UnionId,
    #[filter(use_alias = true)]
    pub app_id: AppId,
    #[filter(use_alias = true)]
    pub account_id: AccountId,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}
// impl Related<crate::profile::Entity> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
