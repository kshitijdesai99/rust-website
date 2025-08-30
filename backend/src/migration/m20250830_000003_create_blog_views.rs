use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum BlogViews {
    Table,
    Id,
    PostId,
    IpAddress,
    Timestamp,
}

#[derive(DeriveIden)]
enum Blogs {
    Table,
    Id,
}

pub struct Migration;

#[async_trait::async_trait]
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20250830_000003_create_blog_views" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlogViews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogViews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BlogViews::PostId).integer().not_null())
                    .col(ColumnDef::new(BlogViews::IpAddress).string().not_null())
                    .col(
                        ColumnDef::new(BlogViews::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BlogViews::Table, BlogViews::PostId)
                            .to(Blogs::Table, Blogs::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_blog_views_post_id")
                    .table(BlogViews::Table)
                    .col(BlogViews::PostId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlogViews::Table).to_owned())
            .await
    }
}
