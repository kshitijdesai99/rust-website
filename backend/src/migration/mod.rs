pub use sea_orm_migration::prelude::*;

mod m20240101_000001_initial;
mod m20250830_000002_create_blogs;
mod m20250830_000003_create_blog_views;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_initial::Migration),
            Box::new(m20250830_000002_create_blogs::Migration),
            Box::new(m20250830_000003_create_blog_views::Migration),
        ]
    }
}
