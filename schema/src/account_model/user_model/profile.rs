use crate::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Filter)]
#[sea_orm(table_name = "profile")]
#[graphql(complex)]
#[graphql(name = "Profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub address_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    //     #[sea_orm(
    //         belongs_to = "super::address::Entity",
    //         from = "Column::AddressId",
    //         to = "super::address::Column::AddressId",
    //         on_update = "Cascade",
    //         on_delete = "NoAction"
    //     )]
    //     Address,
    //     #[sea_orm(
    //         belongs_to = "super::user::Entity",
    //         on_update = "Cascade",
    //         on_delete = "Cascade"
    //     )]
    //     User,
    // }

    // // impl Related<super::address::Entity> for Entity {
    // //     fn to() -> RelationDef {
    // //         Relation::Address.def()
    // //     }
    // // }
    // impl Related<super::user::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::User.def()
    //     }
}

impl ActiveModelBehavior for ActiveModel {}
