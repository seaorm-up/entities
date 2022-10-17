use crate::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Debug, QueryRoot)]
#[seaography(entity = "user")]
#[seaography(entity = "profile")]
pub struct QueryRoot;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // // Replace the sample below with your own migration scripts
        // todo!();
        let db_postgres = sea_orm::DbBackend::Sqlite;
        // let schema = get_schema(QueryRoot).await;
        let schema = sea_orm::Schema::new(db_postgres);

        manager
            .create_table(schema.create_table_from_entity(user::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(profile::Entity))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // manager
        // .drop_table(schema.drop(user::Entity))
        // .await?;
        // Ok(())

        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(profile::Entity).to_owned())
            .await?;
        Ok(())
    }
}
