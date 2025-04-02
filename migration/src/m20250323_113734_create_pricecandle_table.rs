use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PriceCandle::Table)
                    .if_not_exists()
                    .col(pk_auto(PriceCandle::Id)) // Auto-incrementing primary key
                    .col(string(PriceCandle::Token)) // String for token
                    .col(float(PriceCandle::Open)) // f64 for open price
                    .col(float(PriceCandle::High)) // f64 for high price
                    .col(float(PriceCandle::Low)) // f64 for low price
                    .col(float(PriceCandle::Close)) // f64 for close price
                    .col(big_integer(PriceCandle::Timestamp)) // i64 for timestamp
                    .col(string(PriceCandle::Period)) // String for period
                    .col(big_integer(PriceCandle::ChainId)) // i64 for chain ID
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PriceCandle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PriceCandle {
    Table,
    Id,
    Token,
    Open,
    High,
    Low,
    Close,
    Timestamp,
    Period,
    ChainId,
}