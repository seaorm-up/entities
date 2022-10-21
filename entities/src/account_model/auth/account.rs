use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "account")]
#[graphql(complex)]
#[graphql(name = "Account")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    pub account_lock_reason: Option<String>,
    pub account_lock_until: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    //     #[sea_orm(has_one = "super::profile::Entity")]
    //     Profile,
    // }
    // impl Related<crate::profile::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::Profile.def()
    //     }
}

impl ActiveModelBehavior for ActiveModel {}
