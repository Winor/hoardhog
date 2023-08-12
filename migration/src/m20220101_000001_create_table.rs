use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Items::Unit).string().default("Piece"))
                    .col(ColumnDef::new(Items::Description).text())
                    .col(ColumnDef::new(Items::Instock).boolean().default(true))
                    .to_owned(),
            )
            .await?;
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
            // Tags
            manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tags::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Tags::ItemId).integer().default(Value::Int(None)))
                    .col(ColumnDef::new(Tags::GroupId).integer().default(Value::Int(None)))
                    .col(ColumnDef::new(Tags::Description).text())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tags::Table, Tags::ItemId)
                            .to(Items::Table, Items::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tags::Table, Tags::GroupId)
                            .to(Groups::Table, Groups::Id)
                    )
                    .to_owned(),
            )
            .await?;
            // ItemLocation
            manager
            .create_table(
                Table::create()
                    .table(ItemLocation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ItemLocation::ItemId).integer().default(Value::Int(None)))
                    .col(ColumnDef::new(ItemLocation::LocationId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemLocation::Table, ItemLocation::ItemId)
                            .to(Items::Table, Items::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemLocation::Table, ItemLocation::LocationId)
                            .to(Locations::Table, Locations::Id)
                    )
                    .primary_key(Index::create().col(ItemLocation::ItemId).col(ItemLocation::LocationId))
                    .to_owned(),
            )
            .await?;
            // GroupLocation
            manager
            .create_table(
                Table::create()
                    .table(GroupLocation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GroupLocation::GroupId).integer().default(Value::Int(None)))
                    .col(ColumnDef::new(GroupLocation::LocationId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupLocation::Table, GroupLocation::GroupId)
                            .to(Groups::Table, Groups::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupLocation::Table, GroupLocation::LocationId)
                            .to(Locations::Table, Locations::Id)
                    )
                    .primary_key(Index::create().col(GroupLocation::GroupId).col(GroupLocation::LocationId))
                    .to_owned(),
            )
            .await?;
            // ItemGroup
            manager
            .create_table(
                Table::create()
                    .table(ItemGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ItemGroup::ItemId).integer().default(Value::Int(None)))
                    .col(ColumnDef::new(ItemGroup::GroupId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemGroup::Table, ItemGroup::ItemId)
                            .to(Items::Table, Items::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemLocation::Table, ItemGroup::GroupId)
                            .to(Groups::Table, Groups::Id)
                    )
                    .primary_key(Index::create().col(ItemGroup::ItemId).col(ItemGroup::GroupId))
                    .to_owned(),
            )
            .await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Locations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ItemLocation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GroupLocation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ItemGroup::Table).to_owned())
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
    Unit,
    Description,
    Instock,
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
enum Tags {
    Table,
    Id,
    ItemId,
    GroupId,
    Description,
}

#[derive(Iden)]
enum ItemLocation {
    Table,
    ItemId,
    LocationId,
}

#[derive(Iden)]
enum GroupLocation {
    Table,
    GroupId,
    LocationId,
}

#[derive(Iden)]
enum ItemGroup {
    Table,
    ItemId,
    GroupId,
}