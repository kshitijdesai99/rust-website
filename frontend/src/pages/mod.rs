pub mod landing;
pub mod health;
pub mod blogs_list;
pub mod blog_detail;
pub mod not_found;

pub use landing::LandingPage;
pub use health::HealthPage;
pub use blogs_list::BlogsListPage;
pub use blog_detail::{BlogDetailPage, BlogDetailProps};
pub use not_found::NotFoundPage;
