use leptos::*;
use leptos::html::ElementChild;
use leptos::prelude::ClassAttribute;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
        <Router>
            <nav class="bg-blue-600 text-white p-4">
                <div class="container mx-auto flex justify-between">
                    <h1 class="text-xl font-bold">"Gestionnaire de Bibliothèque"</h1>
                    <div class="space-x-4">
                        <a href="/" class="hover:underline"> "Accueil"</a>
                        <a href="/collections" class="hover:underline"> "Collections"</a>
                        <a href="/loans" class="hover:underline">"Prêts"</a>
                        <a href="/authorities" class="hover:underline">"Autorités"</a>
                        <a href="/admin" class="hover:underline">"Admin"</a>
                    </div>
                </div>
            </nav>
            
            <main class="container mx-auto p-4">
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Dashboard/>
                    <Route path=path!("/collections") view=CollectionsPage/>
                    <Route path=path!("/loans") view=LoansPage/>
                    <Route path=path!("/authorities") view=AuthoritiesPage/>
                    <Route path=path!("/admin") view=AdminPage/>
                </Routes>
            </main>
        </Router>
        </div>
    }
}
