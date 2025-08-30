use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;
use crate::services::api::{ApiService, BlogListItem, BlogsListResponse, BlogDetailResponse};

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

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="min-h-screen bg-black flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-6xl font-bold bg-gradient-to-r from-cyan-400 via-purple-500 to-pink-500 bg-clip-text text-transparent drop-shadow-2xl animate-pulse">
                    {"Landing Page"}
                </h1>
                <div class="absolute inset-0 -z-10">
                    <div class="absolute top-1/4 left-1/4 w-72 h-72 bg-cyan-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob"></div>
                    <div class="absolute top-1/3 right-1/4 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000"></div>
                    <div class="absolute bottom-1/4 left-1/3 w-72 h-72 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000"></div>
                </div>
            </div>
        </div>
    }
}

#[function_component(HealthPage)]
pub fn health_page() -> Html {
    let status = use_state(|| "Ready".to_string());
    let loading = use_state(|| false);

    let status_clone = status.clone();
    let loading_clone = loading.clone();
    
    let onclick = Callback::from(move |_| {
        let status = status_clone.clone();
        let loading = loading_clone.clone();
        
        loading.set(true);
        status.set("Checking...".to_string());
        
        wasm_bindgen_futures::spawn_local(async move {
            match ApiService::health_check().await {
                Ok(response) => {
                    status.set(format!("Backend is healthy! Status: {}", response.status));
                },
                Err(e) => {
                    status.set(format!("Connection failed: {}", e));
                }
            }
            loading.set(false);
        });
    });

    html! {
        <div class="min-h-screen bg-black flex items-center justify-center p-8">
            <div class="max-w-md w-full space-y-8 text-center">
                <div>
                    <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500 mb-4">
                        {"Health Check"}
                    </h1>
                    <p class="text-cyan-300 text-lg">{"Click the button to test backend connectivity"}</p>
                </div>
                
                <button 
                    onclick={onclick}
                    disabled={*loading}
                    class="w-full py-4 px-8 text-lg font-semibold rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-opacity-50 disabled:opacity-50 disabled:cursor-not-allowed bg-gradient-to-r from-cyan-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white shadow-lg hover:shadow-cyan-500/25"
                >
                    if *loading {
                        <span class="flex items-center justify-center">
                            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            {"Checking..."}
                        </span>
                    } else {
                        {"Check Health"}
                    }
                </button>
                
                <div class="p-6 rounded-lg border border-cyan-500/30 bg-gray-900/50 backdrop-blur-sm">
                    <div class="text-sm text-cyan-400 mb-2">{"Status:"}</div>
                    <div class="text-lg font-mono text-white">{&*status}</div>
                </div>
                
                <div class="absolute inset-0 -z-10">
                    <div class="absolute top-1/4 left-1/4 w-64 h-64 bg-cyan-500 rounded-full mix-blend-multiply filter blur-xl opacity-10 animate-pulse"></div>
                    <div class="absolute bottom-1/4 right-1/4 w-64 h-64 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-10 animate-pulse"></div>
                </div>
            </div>
        </div>
    }
}

#[function_component(BlogsListPage)]
pub fn blogs_list_page() -> Html {
    let loading = use_state(|| true);
    let error = use_state(|| None as Option<String>);
    let data = use_state(|| None as Option<BlogsListResponse>);
    let page = use_state(|| 1u64);
    let per_page: u64 = 10;

    {
        let loading = loading.clone();
        let error = error.clone();
        let data = data.clone();
        use_effect_with(*page, move |p| {
            let loading = loading.clone();
            let error = error.clone();
            let data = data.clone();
            let current_page = *p;
            loading.set(true);
            error.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::list_blogs(current_page, per_page).await {
                    Ok(resp) => { data.set(Some(resp)); }
                    Err(e) => { error.set(Some(e)); }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_prev = {
        let page = page.clone();
        Callback::from(move |_| {
            if *page > 1 { page.set(*page - 1); }
        })
    };

    let on_next = {
        let page = page.clone();
        let data = data.clone();
        Callback::from(move |_| {
            if let Some(d) = (*data).clone() {
                if *page < d.total_pages { page.set(*page + 1); }
            }
        })
    };

    html! {
        <div class="min-h-screen bg-black text-white p-6">
            <div class="max-w-5xl mx-auto space-y-6">
                <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500">{"Blogs"}</h1>

                if *loading {
                    <div class="text-cyan-300">{"Loading..."}</div>
                } else if let Some(err) = &*error {
                    <div class="text-red-400">{format!("Error: {}", err)}</div>
                } else if let Some(resp) = &*data {
                    <>
                        <div class="grid grid-cols-1 gap-6">
                            { for resp.items.iter().map(|item| html!{ <BlogCard item={item.clone()} /> }) }
                        </div>
                        <div class="flex items-center justify-between mt-6">
                            <button onclick={on_prev}
                                class="px-4 py-2 rounded bg-gray-800 hover:bg-gray-700 disabled:opacity-50"
                                disabled={*page <= 1}
                            >{"Previous"}</button>
                            <div class="text-sm text-cyan-300">{format!("Page {} of {}", resp.page, resp.total_pages)}</div>
                            <button onclick={on_next}
                                class="px-4 py-2 rounded bg-gray-800 hover:bg-gray-700 disabled:opacity-50"
                                disabled={resp.total_pages == 0 || *page >= resp.total_pages}
                            >{"Next"}</button>
                        </div>
                    </>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub item: BlogListItem,
}

#[function_component(BlogCard)]
pub fn blog_card(props: &BlogCardProps) -> Html {
    let item = &props.item;
    html! {
        <div class="p-6 rounded-lg border border-cyan-500/30 bg-gray-900/40 hover:bg-gray-900/60 transition">
            <h2 class="text-2xl font-semibold mb-2">
                <Link<Route> to={Route::BlogDetail { slug: item.slug.clone() }} classes="text-cyan-300 hover:underline">{ &item.title }</Link<Route>>
            </h2>
            if let Some(excerpt) = &item.excerpt {
                <p class="text-gray-300">{ excerpt }</p>
            }
            <div class="mt-3 text-sm text-gray-400">
                { match item.published_at {
                    Some(dt) => format!("Published: {}", dt.format("%Y-%m-%d %H:%M")),
                    None => "Unpublished".to_string(),
                }}
            </div>
        </div>
    }
}

#[function_component(BlogDetailPage)]
pub fn blog_detail_page(props: &BlogDetailProps) -> Html {
    let loading = use_state(|| true);
    let error = use_state(|| None as Option<String>);
    let blog = use_state(|| None as Option<BlogDetailResponse>);

    {
        let loading = loading.clone();
        let error = error.clone();
        let blog = blog.clone();
        let slug = props.slug.clone();
        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);
            let slug = slug.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::get_blog_by_slug(&slug).await {
                    Ok(resp) => blog.set(Some(resp)),
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="min-h-screen bg-black text-white p-6">
            <div class="max-w-3xl mx-auto space-y-6">
                if *loading {
                    <div class="text-cyan-300">{"Loading..."}</div>
                } else if let Some(err) = &*error {
                    <div class="text-red-400">{format!("Error: {}", err)}</div>
                } else if let Some(b) = &*blog {
                    <>
                        <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500">{ &b.title }</h1>
                        <div class="text-sm text-gray-400">
                            { match b.published_at {
                                Some(dt) => format!("Published: {}", dt.format("%Y-%m-%d %H:%M")),
                                None => "Unpublished".to_string(),
                            }}
                        </div>
                        if let Some(excerpt) = &b.excerpt {
                            <p class="text-gray-300 italic">{ excerpt }</p>
                        }
                        <article class="prose prose-invert max-w-none">
                            { Html::from_html_unchecked(AttrValue::from(b.content.clone())) }
                        </article>
                    </>
                } else {
                    <div class="text-gray-300">{"Blog not found"}</div>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BlogDetailProps {
    pub slug: String,
}

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="min-h-screen bg-black flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-9xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-red-500 via-pink-500 to-purple-500 drop-shadow-2xl animate-pulse mb-4">
                    {"404"}
                </h1>
                <p class="text-cyan-300 text-xl mb-8">{"Page not found in the neon void"}</p>
                <div class="absolute inset-0 -z-10">
                    <div class="absolute top-1/4 left-1/3 w-96 h-96 bg-red-500 rounded-full mix-blend-multiply filter blur-xl opacity-15 animate-pulse"></div>
                    <div class="absolute bottom-1/3 right-1/3 w-96 h-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl opacity-15 animate-pulse"></div>
                </div>
            </div>
        </div>
    }
}
