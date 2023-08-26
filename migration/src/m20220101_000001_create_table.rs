use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // Location
            manager
            .create_table(
                Table::create()
                    .table(Locations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Locations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Locations::Name).string())
                    .col(ColumnDef::new(Locations::Description).text())
                    .to_owned(),
            )
            .await?;
            // Categories
            manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Categories::Name).string())
                    .col(ColumnDef::new(Categories::Fields).string())
                    .to_owned(),
            )
            .await?;
        //Item
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Items::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Items::Name).string())
                    .col(ColumnDef::new(Items::Quantity).integer().default(1))
                    .col(ColumnDef::new(Items::CategoryId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Items::Table, Items::CategoryId)
                            .to(Categories::Table, Categories::Id)
                    )
                    .col(ColumnDef::new(Items::Description).text())
                    .col(ColumnDef::new(Items::LocationId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Items::Table, Items::LocationId)
                            .to(Locations::Table, Locations::Id)
                    )
                    .to_owned(),
            )
            .await?;
            // Groups
            manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Groups::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Groups::Name).string())
                    .col(ColumnDef::new(Groups::Description).text())
                    .to_owned(),
            )
            .await?;
            // Categories
            manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Categories::Fields).json())
                    .to_owned(),
            )
            .await?;
        // item group
            manager
            .create_table(
                Table::create()
                    .table(ItemGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ItemGroup::ItemId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemGroup::Table, ItemGroup::ItemId)
                            .to(Items::Table, Items::Id)
                    )
                    .col(ColumnDef::new(ItemGroup::GroupId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemGroup::Table, ItemGroup::GroupId)
                            .to(Groups::Table, Groups::Id)
                    )
                    .primary_key(Index::create().col(ItemGroup::ItemId).col(ItemGroup::GroupId))
                    .to_owned(),
            )
            .await?;
        // Field
        manager
        .create_table(
            Table::create()
                .table(Field::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Categories::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Field::Data).json())
                .to_owned(),
        )
        .await?;
        // item field
        manager
        .create_table(
            Table::create()
                .table(ItemField::Table)
                .if_not_exists()
                .col(ColumnDef::new(ItemField::ItemId).integer().default(Value::Int(None)))
                .foreign_key(
                    ForeignKey::create()
                        .from(ItemField::Table, ItemField::ItemId)
                        .to(Items::Table, Items::Id)
                )
                .col(ColumnDef::new(ItemField::FieldId).integer().default(Value::Int(None)))
                .foreign_key(
                    ForeignKey::create()
                        .from(ItemField::Table, ItemField::FieldId)
                        .to(Field::Table, Field::Id)
                )
                .primary_key(Index::create().col(ItemField::ItemId).col(ItemField::FieldId))
                .to_owned(),
        )
        .await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
        .drop_table(Table::drop().table(Locations::Table).to_owned())
        .await?;
        manager
        .drop_table(Table::drop().table(Categories::Table).to_owned())
        .await?;
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ItemGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Field::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ItemField::Table).to_owned())
            .await?;
        
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Items {
    Table,
    Id,
    Name,
    Quantity,
    CategoryId,
    Description,
    LocationId,
}

#[derive(Iden)]
enum Locations {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum Groups {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Fields,
    Name,
}

#[derive(Iden)]
enum ItemField {
    Table,
    ItemId,
    FieldId,
}

#[derive(Iden)]
enum Field {
    Table,
    Id,
    Data,
}

#[derive(Iden)]
enum ItemGroup {
    Table,
    GroupId,
    ItemId,
}