use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "blogs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    #[sea_orm(unique)]
    pub slug: String,
    pub excerpt: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub status: Option<String>,
    pub published_at: Option<DateTimeUtc>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::AuthorId", to = "super::users::Column::Id")]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

impl ActiveModelBehavior for ActiveModel {}
