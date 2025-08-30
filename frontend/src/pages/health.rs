use yew::prelude::*;
use crate::services::health::health_check;

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
            match health_check().await {
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
