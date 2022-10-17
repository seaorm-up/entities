use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject, Filter)]
#[sea_orm(table_name = "user")]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub avatar: Option<String>,
    pub account_lock_reason: Option<String>,
    pub account_lock_until: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::profile::Entity",
        from = "Column::Id",
        to = "crate::profile::Column::Id"
    )]
    Profile,
}
impl Related<crate::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
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
