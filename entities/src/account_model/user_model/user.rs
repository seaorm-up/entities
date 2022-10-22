use crate::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user")]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[filter(use_alias = true)]
    pub id: AccountId,
    // pub avatar: Option<String>,
    // pub account_lock_reason: Option<String>,
    // pub account_lock_until: Option<DateTimeUtc>,
    // pub created_at: DateTimeUtc,
    // pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user_profile::Entity",
        from = "Column::Id",
        to = "crate::user_profile::Column::Id"
    )]
    UserProfile,
    #[sea_orm(
        belongs_to = "crate::account::Entity",
        from = "Column::Id",
        to = "crate::account::Column::Id"
    )]
    Account,
    #[sea_orm(
        belongs_to = "crate::org_account::Entity",
        from = "Column::Id",
        to = "crate::org_account::Column::OrgId"
    )]
    OrgAccount,
}
impl Related<crate::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}
impl Related<crate::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}
impl Related<crate::org_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgAccount.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}

#[tokio::test]
async fn test_querying() {
    use crate::*;
    #[derive(Debug, QueryRoot)]
    #[seaography(entity = "crate::user")]
    #[seaography(entity = "crate::profile")]
    pub struct QueryRoot;
    let schema = get_schema(QueryRoot).await;
    let obj = schema
        .execute(
            r#"
      {
        profile(pagination: {limit: 2, page: 1}) {
            data{
                id
                name
                # lastName
            }

        }
      }
      "#,
        )
        .await;
    println!("{:#?}", obj);
}
