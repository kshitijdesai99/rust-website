use yew::prelude::*;
use yew_router::prelude::*;
use crate::app_routes::Route;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <section class="min-h-screen bg-black text-white">
            <div class="relative mx-auto max-w-7xl px-6 py-20 lg:py-28">
                <div class="absolute inset-0 -z-10 overflow-hidden">
                    <div class="absolute -top-24 -left-24 h-72 w-72 rounded-full bg-cyan-500 blur-3xl opacity-20"></div>
                    <div class="absolute -bottom-24 -right-24 h-72 w-72 rounded-full bg-purple-500 blur-3xl opacity-20"></div>
                </div>

                <div class="grid grid-cols-1 items-center gap-12 lg:grid-cols-2">
                    <div class="space-y-6">
                        <h1 class="text-5xl font-bold tracking-tight sm:text-6xl">
                            <span class="bg-gradient-to-r from-cyan-400 via-purple-500 to-pink-500 bg-clip-text text-transparent">{"Kshitij's Rust Web"}</span>
                        </h1>
                        <p class="text-lg text-gray-300 max-w-xl">{"AI + Engineering + System Design + Philosophy"}</p>
                        <div class="flex items-center gap-4">
                            <Link<Route> to={Route::Blogs}
                                classes="inline-flex items-center justify-center rounded-lg bg-gradient-to-r from-cyan-500 to-purple-600 px-5 py-3 text-base font-semibold text-white shadow-lg shadow-cyan-500/20 transition hover:from-cyan-400 hover:to-purple-500">
                                {"Read the Blog"}
                            </Link<Route>>
                        </div>
                    </div>

                    <div class="relative">
                        <div class="pointer-events-none absolute inset-0 -z-10 rounded-3xl bg-gradient-to-br from-cyan-500/20 to-purple-500/20 blur-2xl"></div>
                        <img
                            src="/assets/tiger.png"
                            alt="Neon tiger"
                            class="mx-auto w-full max-w-md rounded-2xl ring-1 ring-white/10 shadow-2xl"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}
