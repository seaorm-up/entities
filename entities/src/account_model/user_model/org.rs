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
    #[filter(use_alias = true)]
    pub id: OpenDepartmentId,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub name: String,
    pub repository: Option<String>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::org_account::Entity",
        from = "Column::Id",
        to = "crate::org_account::Column::OrgId"
    )]
    OrgAccount,
}
impl Related<crate::org_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgAccount.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
