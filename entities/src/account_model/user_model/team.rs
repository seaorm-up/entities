use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "team")]
#[graphql(complex)]
#[graphql(name = "Team")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub org_id: Option<u32>,
    pub user_team_id: u64,
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
