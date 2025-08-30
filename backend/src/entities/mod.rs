pub mod users;
pub mod blogs;
pub mod blog_views;

pub use users::Entity as Users;
pub use blogs::Entity as Blogs;
pub use blog_views::Entity as BlogViews;

seaography::register_entity_modules!([users, blogs, blog_views]);
