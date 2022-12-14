use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "city")]
#[graphql(complex)]
#[graphql(name = "City")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub city_id: u32,
    pub city: String,
    pub country_id: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::country::Entity",
        from = "Column::CountryId",
        to = "super::country::Column::CountryId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Country,
    #[sea_orm(has_many = "super::address::Entity")]
    Address,
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
