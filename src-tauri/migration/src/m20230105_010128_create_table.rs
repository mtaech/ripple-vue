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
                    .col(ColumnDef::new(Chapter::TextContent).string())
                    .col(ColumnDef::new(Chapter::HtmlContent).string())
                    .col(ColumnDef::new(Chapter::TextCount).integer())
                    .col(ColumnDef::new(Chapter::VolumeId).string())
                    .col(ColumnDef::new(Chapter::VolumeName).string())
                    .col(ColumnDef::new(Chapter::BookId).string())
                    .col(ColumnDef::new(Chapter::BookName).string())
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
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Book::Name).string())
                    .col(ColumnDef::new(Book::Description).string())
                    .col(ColumnDef::new(Book::CreateTime).string())
                    .col(ColumnDef::new(Book::ModifiedTime).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Set::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Set::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Set::Name).string())
                    .col(ColumnDef::new(Set::Description).string())
                    .col(ColumnDef::new(Set::CreateTime).string())
                    .col(ColumnDef::new(Set::ModifiedTime).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .col(
                        ColumnDef::new(Attachment::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Attachment::FileName).string())
                    .col(ColumnDef::new(Attachment::FileSize).integer())
                    .col(ColumnDef::new(Attachment::FilePath).string())
                    .col(ColumnDef::new(Attachment::Suffix).string())
                    .col(ColumnDef::new(Attachment::ContentType).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(AttRel::Table)
                    .col(ColumnDef::new(AttRel::AttachId).string())
                    .col(ColumnDef::new(AttRel::ModelId).string())
                    .col(ColumnDef::new(AttRel::ModelName).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(BookVolume::Table)
                    .col(
                        ColumnDef::new(BookVolume::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookVolume::Name).string())
                    .col(ColumnDef::new(BookVolume::BookId).string())
                    .col(ColumnDef::new(BookVolume::BookName).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(Iden)]
enum Chapter {
    Table,
    Id,
    Name,
    TextContent,
    HtmlContent,
    TextCount,
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

#[derive(Iden)]
enum Book {
    Table,
    Id,
    Name,
    Description,
    CreateTime,
    ModifiedTime,
}

#[derive(Iden)]
enum Set {
    Table,
    Id,
    Name,
    Description,
    CreateTime,
    ModifiedTime,
}

#[derive(Iden)]
enum Attachment {
    Table,
    Id,
    FileName,
    FileSize,
    FilePath,
    Suffix,
    ContentType,
}

#[derive(Iden)]
enum AttRel {
    Table,
    AttachId,
    ModelId,
    ModelName,
}

#[derive(Iden)]
enum BookVolume {
    Table,
    Id,
    Name,
    BookId,
    BookName,
}
