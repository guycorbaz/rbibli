use leptos::*;
use leptos::prelude::ElementChild;
use leptos::prelude::ClassAttribute;

#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-bold">"Administration"</h1>
            <div class="bg-white rounded-lg shadow p-6">
                <p class="text-gray-600">"Administration à venir ..."</p>
            </div>
        </div>
    }
}
