use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-bold">"Tableau de Bord"</h1>
            <div class="bg-white rounded-lg shadow p-6">
                <p class="text-gray-600">"Bienvenue dans votre gestionnaire de bibliothèque personnelle !"</p>
            </div>
        </div>
    }
}
