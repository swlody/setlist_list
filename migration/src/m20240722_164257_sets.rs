use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Sets::Table)
                    .col(pk_auto(Sets::Id))
                    .col(string_null(Sets::BandName))
                    .col(date(Sets::Date))
                    .col(string_null(Sets::Venue))
                    .col(json_null(Sets::Setlist))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sets {
    Table,
    Id,
    BandName,
    Date,
    Venue,
    Setlist,
}
