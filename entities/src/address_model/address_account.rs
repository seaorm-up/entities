use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "address_account")]
#[graphql(complex)]
#[graphql(name = "AddressAccount")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub address_id: u64,
    pub account_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        has_many = "crate::address::Entity",
        from = "Column::AddressId",
        to = "crate::address::Column::Id"
    )]
    Address,
    #[sea_orm(
        has_many = "crate::account::Entity",
        from = "Column::AccountId",
        to = "crate::account::Column::Id"
    )]
    Account,
}
impl Related<crate::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}
impl Related<crate::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
