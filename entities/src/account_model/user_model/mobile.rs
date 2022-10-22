use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "mobile")]
#[graphql(complex)]
#[graphql(name = "Mobile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub mobile: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::mobile_account::Entity",
        from = "Column::Mobile",
        to = "crate::mobile_account::Column::Mobile"
    )]
    MobileAccount,
}
impl Related<crate::mobile_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MobileAccount.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
