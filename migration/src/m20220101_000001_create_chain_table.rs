use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chain::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chain::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Chain::RPC).string().not_null())
                    .col(ColumnDef::new(Chain::NetworkID).string())
                    .col(ColumnDef::new(Chain::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chain::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Chain {
    Table,
    Id,
    Name,
    RPC,
    NetworkID,
    CreatedAt,
}
