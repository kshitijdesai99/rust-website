use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use crate::services::blogs::{get_blog_by_slug, BlogDetailResponse};

#[derive(Properties, PartialEq)]
pub struct BlogDetailProps {
    pub slug: String,
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
                match get_blog_by_slug(&slug).await {
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
                    <div class="flex items-center gap-3 text-cyan-300">
                        <svg class="animate-spin -ml-1 mr-3 h-6 w-6 text-cyan-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <span class="text-lg">{"Loading..."}</span>
                    </div>
                } else if let Some(err) = &*error {
                    <div class="text-red-400">{format!("Error: {}", err)}</div>
                } else if let Some(b) = &*blog {
                    <>
                        <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500">{ &b.title }</h1>
                        <div class="text-sm text-cyan-300 font-medium">{ format!("Views: {}", b.views_count) }</div>
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
