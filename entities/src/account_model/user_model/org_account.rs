use crate::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "org_account")]
#[graphql(complex)]
#[graphql(name = "OrgAccount")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: u64,
    #[filter(use_alias = true)]
    pub org_id: OpenDepartmentId,
    #[filter(use_alias = true)]
    pub account_id: AccountId,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        has_many = "crate::org::Entity",
        from = "Column::OrgId",
        to = "crate::org::Column::Id"
    )]
    Org,
    #[sea_orm(
        has_many = "crate::user::Entity",
        from = "Column::AccountId",
        to = "crate::user::Column::Id"
    )]
    User,
    #[sea_orm(
        has_many = "crate::team::Entity",
        from = "Column::AccountId",
        to = "crate::team::Column::Id"
    )]
    Team,
}

impl Related<crate::org::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<crate::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
