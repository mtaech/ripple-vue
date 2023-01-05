use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chapter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chapter::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chapter::Name).string())
                    .col(ColumnDef::new(Chapter::Content).string())
                    .col(ColumnDef::new(Chapter::VolumeId).string())
                    .col(ColumnDef::new(Chapter::VolumeName).string())
                    .col(ColumnDef::new(Chapter::CreateTime).string())
                    .col(ColumnDef::new(Chapter::ModifiedTime).string())
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Character::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).string())
                    .col(ColumnDef::new(Character::Age).string())
                    .col(ColumnDef::new(Character::Gender).string())
                    .col(ColumnDef::new(Character::CreateTime).string())
                    .col(ColumnDef::new(Character::ModifiedTime).string())
                    .to_owned(),
            )
            .await
    }

    /* async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Chapter::Table).to_owned())
            .await
    }*/
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Chapter {
    Table,
    Id,
    Name,
    Content,
    VolumeId,
    VolumeName,
    BookId,
    BookName,
    CreateTime,
    ModifiedTime,
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Name,
    Age,
    Gender,
    CreateTime,
    ModifiedTime,
}
