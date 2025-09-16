use leptos::*;
use leptos::prelude::*;

#[component]
pub fn AuthoritiesPage() -> impl IntoView {
    view! {
        <div class="flex">
            <AuthoritiesSideMenu/>
                <div class="p-6 space-y-6">
                    <h1 class="text-2xl font-bold">"Autorités"</h1>
                    <div class="bg-white rounded-lg shadow p-6">
                        <p class="text-gray-600">"Autorités à venir..."</p>
                    </div>
                </div>

        </div>
    }
}


#[component]
pub fn AuthoritiesSideMenu() -> impl IntoView {
    let handle_new_author = move |_| {
        // Logique pour créer un nouvel auteur
        log::info!("Nouvel auteur cliqué");
    };

    let handle_new_editor = move |_| {
        // Logique pour créer un nouvel éditeur
        log::info!("Nouvel éditeur cliqué");
    };

    view! {
        <div class=" left-0 top-0 h-full bg-white border-r border-gray-200 shadow-lg z-40 w-64">
            // En-tête du menu
            <div class="p-4 border-b border-gray-200">
                <h2 class="text-lg font-semibold text-gray-800">"Autorités"</h2>
            </div>

            // Contenu du menu
            <div class="p-4 space-y-2">
                // Bouton "Nouvel auteur"
                <button
                    on:click=handle_new_author
                    class="w-full flex items-center space-x-3 px-4 py-3 text-left rounded-md hover:bg-blue-50 hover:text-blue-700 transition-colors duration-200 group"
                >
                    <div class="flex-shrink-0">
                        <svg class="w-5 h-5 text-gray-400 group-hover:text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                        </svg>
                    </div>
                    <span class="text-sm font-medium">"Nouvel auteur"</span>
                    <div class="flex-1"></div>
                    <svg class="w-4 h-4 text-gray-300 group-hover:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                    </svg>
                </button>

                // Bouton "Nouvel éditeur"
                <button
                    on:click=handle_new_editor
                    class="w-full flex items-center space-x-3 px-4 py-3 text-left rounded-md hover:bg-green-50 hover:text-green-700 transition-colors duration-200 group"
                >
                    <div class="flex-shrink-0">
                        <svg class="w-5 h-5 text-gray-400 group-hover:text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-2m-2 0H5m14 0v-5c0-1.1-.9-2-2-2s-2 .9-2 2v5m-4 0V9a2 2 0 012-2h2a2 2 0 012 2v12m-6 0V9"/>
                        </svg>
                    </div>
                    <span class="text-sm font-medium">"Nouvel éditeur"</span>
                    <div class="flex-1"></div>
                    <svg class="w-4 h-4 text-gray-300 group-hover:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                    </svg>
                </button>
            </div>

            // Section supplémentaire
            <div class="border-t border-gray-200 mt-4 pt-4 p-4">
                <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-3">"Actions rapides"</h3>
                <div class="space-y-2">
                    <a href="/authorities" class="block text-sm text-gray-600 hover:text-blue-600 transition-colors">
                        "Tous les auteurs"
                    </a>
                    <a href="/authorities/publishers" class="block text-sm text-gray-600 hover:text-blue-600 transition-colors">
                        "Tous les éditeurs"
                    </a>
                </div>
            </div>
        </div>
    }
}
