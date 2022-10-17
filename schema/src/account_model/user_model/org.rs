use crate::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "org")]
#[graphql(complex)]
#[graphql(name = "Org")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub name: String,
    pub repository: Option<String>,
    // pub created_at: DateTimeUtc,
    // pub updated_at: DateTimeUtc,
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
