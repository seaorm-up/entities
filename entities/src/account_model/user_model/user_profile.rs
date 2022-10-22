use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "user_profile")]
#[graphql(complex)]
#[graphql(name = "UserProfile")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub surname: Option<String>,
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
