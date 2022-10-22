use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "account")]
#[graphql(complex)]
#[graphql(name = "Account")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    pub name: String,
    pub passwrod: String,
    pub account_lock_reason: Option<String>,
    pub account_lock_until: Option<DateTimeUtc>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::profile::Entity",
        from = "Column::Id",
        to = "crate::profile::Column::Id"
    )]
    Profile,
    #[sea_orm(
        belongs_to = "crate::user::Entity",
        from = "Column::Id",
        to = "crate::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "crate::team::Entity",
        from = "Column::Id",
        to = "crate::team::Column::Id"
    )]
    Team,
    #[sea_orm(
        belongs_to = "crate::mobile_account::Entity",
        from = "Column::Id",
        to = "crate::mobile_account::Column::AccountId"
    )]
    MobileAccount,
    #[sea_orm(
        belongs_to = "crate::email_account::Entity",
        from = "Column::Id",
        to = "crate::email_account::Column::AccountId"
    )]
    EmailAccount,
    #[sea_orm(
        belongs_to = "crate::address_account::Entity",
        from = "Column::Id",
        to = "crate::address_account::Column::AccountId"
    )]
    AddressAccount,
}
impl Related<crate::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
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
impl Related<crate::mobile_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MobileAccount.def()
    }
}
impl Related<crate::email_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailAccount.def()
    }
}
impl Related<crate::address_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddressAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
