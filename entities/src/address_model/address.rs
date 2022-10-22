use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "address")]
#[graphql(complex)]
#[graphql(name = "Address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u32,
    pub address: String,
    pub city_id: u32,
    pub district: String,
    pub last_update: DateTimeUtc,
    pub postal_code: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::city::Entity",
        from = "Column::CityId",
        to = "super::city::Column::CityId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    City,
    #[sea_orm(
        belongs_to = "super::address_account::Entity",
        from = "Column::Id",
        to = "super::address_account::Column::AddressId",
        // on_update = "Cascade",
        // on_delete = "NoAction"
    )]
    AddressAccount,
    // #[sea_orm(has_many = "super::customer::Entity")]
    // Customer,
    // #[sea_orm(has_many = "super::staff::Entity")]
    // Staff,
    // #[sea_orm(has_many = "super::store::Entity")]
    // Store,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}
impl Related<super::address_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddressAccount.def()
    }
}

// impl Related<super::customer::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Customer.def()
//     }
// }

// impl Related<super::staff::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Staff.def()
//     }
// }

// impl Related<super::store::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Store.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
