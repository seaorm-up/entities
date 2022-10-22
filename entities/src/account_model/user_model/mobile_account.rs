use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "mobile_account")]
#[graphql(complex)]
#[graphql(name = "MobileAccount")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub mobile: u64,
    pub account_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        has_many = "crate::mobile::Entity",
        from = "Column::Mobile",
        to = "crate::mobile::Column::Mobile"
    )]
    Mobile,
    #[sea_orm(
        has_many = "crate::account::Entity",
        from = "Column::AccountId",
        to = "crate::account::Column::Id"
    )]
    Account,
}
impl Related<crate::mobile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mobile.def()
    }
}
impl Related<crate::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
