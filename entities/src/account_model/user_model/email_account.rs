use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "email_account")]
#[graphql(complex)]
#[graphql(name = "EmailAccount")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub email_id: u64,
    pub account_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        has_many = "crate::email::Entity",
        from = "Column::EmailId",
        to = "crate::email::Column::Id"
    )]
    Email,
    #[sea_orm(
        has_many = "crate::account::Entity",
        from = "Column::AccountId",
        to = "crate::account::Column::Id"
    )]
    Account,
}
impl Related<crate::email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Email.def()
    }
}
impl Related<crate::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
