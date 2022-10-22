use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "team")]
#[graphql(complex)]
#[graphql(name = "Team")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::team_profile::Entity",
        from = "Column::Id",
        to = "crate::team_profile::Column::Id"
    )]
    TeamProfile,
    #[sea_orm(
        belongs_to = "crate::account::Entity",
        from = "Column::Id",
        to = "crate::account::Column::Id"
    )]
    Account,
    #[sea_orm(
        belongs_to = "crate::org_account::Entity",
        from = "Column::Id",
        to = "crate::org_account::Column::OrgId"
    )]
    OrgAccount,
}
impl Related<crate::team_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamProfile.def()
    }
}
impl Related<crate::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}
impl Related<crate::org_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
