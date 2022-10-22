use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "country")]
#[graphql(complex)]
#[graphql(name = "Country")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub country_id: u16,
    pub country: String,
    // pub last_update: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::city::Entity")]
    City,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
