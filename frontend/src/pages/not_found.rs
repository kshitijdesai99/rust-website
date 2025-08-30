use yew::prelude::*;

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
