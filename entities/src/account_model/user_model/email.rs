use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "email")]
#[graphql(complex)]
#[graphql(name = "Email")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::email_account::Entity",
        from = "Column::Id",
        to = "crate::email_account::Column::EmailId"
    )]
    EmailAccount,
}
impl Related<crate::email_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
