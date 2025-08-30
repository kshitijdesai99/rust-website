use yew::prelude::*;
use yew_router::prelude::*;
use crate::services::blogs::{list_blogs, BlogListItem, BlogsListResponse};
use crate::app_routes::Route;

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
                match list_blogs(current_page, per_page).await {
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
                    <div class="flex items-center gap-3 text-cyan-300">
                        <svg class="animate-spin -ml-1 mr-3 h-6 w-6 text-cyan-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <span class="text-lg">{"Loading..."}</span>
                    </div>
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
