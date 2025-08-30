use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    LandingPage,
    HealthPage,
    BlogsListPage,
    BlogDetailPage,
    NotFoundPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/health")]
    Health,
    #[at("/blogs")]
    Blogs,
    #[at("/blogs/:slug")]
    BlogDetail { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::Health => html! { <HealthPage /> },
        Route::Blogs => html! { <BlogsListPage /> },
        Route::BlogDetail { slug } => html! { <BlogDetailPage slug={slug} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
