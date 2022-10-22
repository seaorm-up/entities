use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "profile")]
#[graphql(complex)]
#[graphql(name = "Profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    pub avatar: Option<String>,
    pub description: Option<String>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user_profile::Entity",
        from = "Column::Id",
        to = "crate::user_profile::Column::Id"
    )]
    UserProfile,
    #[sea_orm(
        belongs_to = "crate::team_profile::Entity",
        from = "Column::Id",
        to = "crate::team_profile::Column::Id"
    )]
    TeamProfile,
}

impl Related<crate::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}
impl Related<crate::team_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamProfile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
