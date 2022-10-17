use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "token")]
#[graphql(complex)]
#[graphql(name = "Token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub user_id: i32,
    // token: SecureToken,
    // pub name: String,
    pub token: String,
    pub api_type: String,
    pub created_at: DateTimeUtc,
    pub last_used_at: Option<DateTimeUtc>,
    pub revoked: bool,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    //     #[sea_orm(has_one = "super::profile::Entity")]
    //     Profile,
    // }
    // impl Related<crate::profile::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::Profile.def()
    //     }
}

impl ActiveModelBehavior for ActiveModel {}
