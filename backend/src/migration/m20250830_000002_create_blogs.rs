use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create blogs table
        manager
            .create_table(
                Table::create()
                    .table(Blogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blogs::AuthorId).integer().not_null())
                    .col(ColumnDef::new(Blogs::Title).string().not_null())
                    .col(
                        ColumnDef::new(Blogs::Slug)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Blogs::Excerpt).text())
                    .col(ColumnDef::new(Blogs::Content).text().not_null())
                    .col(
                        ColumnDef::new(Blogs::Status)
                            .string_len(16)
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Blogs::PublishedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Blogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Blogs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_blogs_author")
                            .from(Blogs::Table, Blogs::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Indices
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_blogs_author_id")
                    .table(Blogs::Table)
                    .col(Blogs::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_blogs_status_published")
                    .table(Blogs::Table)
                    .col(Blogs::Status)
                    .col(Blogs::PublishedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blogs {
    Table,
    Id,
    AuthorId,
    Title,
    Slug,
    Excerpt,
    Content,
    Status,
    PublishedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
