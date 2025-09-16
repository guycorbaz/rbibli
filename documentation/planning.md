# Planification de Développement Incrémental - Gestionnaire de Bibliothèque Personnelle

## Vue d'ensemble
Cette planification décompose le développement en très petites étapes facilement réalisables. On commence par créer le frontend Leptos avec du contenu fictif, puis on développe le backend progressivement en l'intégrant au fur et à mesure. Chaque étape peut être développée, testée et déployée indépendamment.

## Phase 1: Frontend avec Données Fictives

### Étape 1.1: Configuration initiale du projet frontend (1 jour)
**Objectif** : Avoir un projet Leptos compilable avec structure de base
- [ ] Créer la structure frontend/
- [ ] Configurer Cargo.toml pour frontend (Leptos + reqwest-wasm)
- [ ] Configurer Trunk.toml pour build WASM
- [ ] Créer "Hello World" en Leptos
- [ ] Vérifier que tout compile et fonctionne
- [ ] Configurer Tailwind CSS

**Livrable** : Application Leptos affichant "Hello World" sur http://localhost:3000

### Étape 1.2: Structure de base et navigation Leptos (1 jour)
**Objectif** : Navigation fonctionnelle entre les pages
- [ ] Configurer leptos_router
- [ ] Créer composant `App` avec navigation
- [ ] Créer pages vides : Dashboard, Titles, Loans, Scan
- [ ] Ajouter menu de navigation avec Tailwind
- [ ] Tester la navigation entre pages

**Exemple de code** :
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="bg-blue-600 text-white p-4">
                <div class="container mx-auto flex justify-between">
                    <h1 class="text-xl font-bold">"Gestionnaire de Bibliothèque"</h1>
                    <div class="space-x-4">
                        <A href="/" class="hover:underline">"Accueil"</A>
                        <A href="/titles" class="hover:underline">"Titres"</A>
                        <A href="/loans" class="hover:underline">"Prêts"</A>
                        <A href="/scan" class="hover:underline">"Scanner"</A>
                    </div>
                </div>
            </nav>
            
            <main class="container mx-auto p-4">
                <Routes>
                    <Route path="/" view=Dashboard/>
                    <Route path="/titles" view=TitlesPage/>
                    <Route path="/loans" view=LoansPage/>
                    <Route path="/scan" view=ScanPage/>
                </Routes>
            </main>
        </Router>
    }
}
```

**Livrable** : Navigation fonctionnelle entre toutes les pages

### Étape 1.3: Page Titres avec données fictives (1-2 jours)
**Objectif** : Afficher une liste de titres avec données en dur
- [ ] Créer modèles `Title` et `Volume` côté frontend
- [ ] Créer données fictives (10-15 titres avec volumes)
- [ ] Composant `TitleCard` avec design Tailwind
- [ ] Composant `TitlesList` affichant les données fictives
- [ ] Gestion des états de chargement avec Suspense

**Exemple de code** :
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: String,
    pub title: String,
    pub isbn: Option<String>,
    pub authors: Vec<String>,
    pub total_volumes: i32,
    pub available_volumes: i32,
}

fn get_mock_titles() -> Vec<Title> {
    vec![
        Title {
            id: "1".to_string(),
            title: "Le Seigneur des Anneaux".to_string(),
            isbn: Some("9782070612888".to_string()),
            authors: vec!["J.R.R. Tolkien".to_string()],
            total_volumes: 3,
            available_volumes: 2,
        },
        // ... plus de données fictives
    ]
}

#[component]
pub fn TitlesPage() -> impl IntoView {
    let titles = create_signal(get_mock_titles());

    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-bold">"Mes Titres"</h1>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <For
                    each=move || titles.0.get()
                    key=|title| title.id.clone()
                    children=move |title| view! { <TitleCard title=title/> }
                />
            </div>
        </div>
    }
}
```

**Livrable** : Page titres fonctionnelle avec design attrayant

### Étape 1.4: Formulaire d'ajout de titre (1 jour)
**Objectif** : Formulaire réactif pour ajouter des titres
- [ ] Composant `TitleForm` avec signaux réactifs
- [ ] Validation côté client
- [ ] Ajout à la liste locale (pas encore de backend)
- [ ] Feedback utilisateur avec notifications toast

**Exemple de code** :
```rust
#[component]
pub fn TitleForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (isbn, set_isbn) = create_signal(String::new());
    let (authors, set_authors) = create_signal(String::new());
    
    let titles = use_context::<RwSignal<Vec<Title>>>().expect("Titles context");

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let new_title = Title {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.get(),
            isbn: if isbn.get().is_empty() { None } else { Some(isbn.get()) },
            authors: authors.get().split(',').map(|s| s.trim().to_string()).collect(),
            total_volumes: 1,
            available_volumes: 1,
        };
        
        titles.update(|titles| titles.push(new_title));
        
        // Reset form
        set_title.set(String::new());
        set_isbn.set(String::new());
        set_authors.set(String::new());
    };

    view! {
        <form on:submit=submit class="space-y-4 bg-white p-6 rounded-lg shadow">
            <h2 class="text-xl font-semibold">"Ajouter un Titre"</h2>
            
            <input 
                prop:value=title
                on:input=move |ev| set_title.set(event_target_value(&ev))
                placeholder="Titre du livre"
                class="w-full px-3 py-2 border rounded-md"
                required
            />
            
            <input 
                prop:value=isbn
                on:input=move |ev| set_isbn.set(event_target_value(&ev))
                placeholder="ISBN (optionnel)"
                class="w-full px-3 py-2 border rounded-md"
            />
            
            <input 
                prop:value=authors
                on:input=move |ev| set_authors.set(event_target_value(&ev))
                placeholder="Auteurs (séparés par des virgules)"
                class="w-full px-3 py-2 border rounded-md"
                required
            />
            
            <button 
                type="submit"
                class="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700"
            >
                "Ajouter"
            </button>
        </form>
    }
}
```

**Livrable** : Formulaire fonctionnel ajoutant des titres à la liste locale

### Étape 1.5: Page Scanner avec interface (1 jour)
**Objectif** : Interface de scan avec feedback visuel
- [ ] Composant `ScanPage` avec champ de saisie
- [ ] Simulation de scan avec données fictives
- [ ] Affichage conditionnel des résultats
- [ ] Design responsive pour tablette

**Exemple de code** :
```rust
#[component]
pub fn ScanPage() -> impl IntoView {
    let (barcode, set_barcode) = create_signal(String::new());
    let (scan_result, set_scan_result) = create_signal(None::<Title>);
    let (loading, set_loading) = create_signal(false);

    let mock_scan = move |code: String| {
        set_loading.set(true);
        
        // Simuler un délai de scan
        set_timeout(move || {
            let result = if code == "VOL-000001" {
                Some(Title {
                    id: "1".to_string(),
                    title: "Le Seigneur des Anneaux".to_string(),
                    isbn: Some("9782070612888".to_string()),
                    authors: vec!["J.R.R. Tolkien".to_string()],
                    total_volumes: 3,
                    available_volumes: 2,
                })
            } else {
                None
            };
            
            set_scan_result.set(result);
            set_loading.set(false);
        }, Duration::from_millis(1000));
    };

    let on_scan = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !barcode.get().is_empty() {
            mock_scan(barcode.get());
            set_barcode.set(String::new());
        }
    };

    view! {
        <div class="max-w-2xl mx-auto space-y-6">
            <h1 class="text-2xl font-bold">"Scanner de Code-barres"</h1>
            
            <div class="bg-white rounded-lg shadow-md p-6">
                <input 
                    prop:value=barcode
                    on:input=move |ev| set_barcode.set(event_target_value(&ev))
                    on:keydown=on_scan
                    placeholder="Scanner ou saisir un code-barres..."
                    class="w-full px-4 py-3 text-lg border rounded-md focus:ring-2 focus:ring-blue-500"
                    autofocus
                />
                
                <Show when=move || loading.get()>
                    <div class="flex items-center justify-center py-8">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                        <span class="ml-2">"Recherche..."</span>
                    </div>
                </Show>
                
                <Show when=move || scan_result.get().is_some()>
                    {move || scan_result.get().map(|title| view! {
                        <div class="mt-6 p-4 bg-green-50 border border-green-200 rounded-md">
                            <h3 class="text-lg font-semibold text-green-800">"Volume trouvé !"</h3>
                            <TitleCard title=title />
                        </div>
                    })}
                </Show>
            </div>
        </div>
    }
}
```

**Livrable** : Interface de scan fonctionnelle avec simulation

### Étape 1.6: Page Prêts avec données fictives (1 jour)
**Objectif** : Afficher les prêts actifs et historique
- [ ] Modèle `Loan` côté frontend
- [ ] Données fictives de prêts
- [ ] Composant `LoanCard` avec statut
- [ ] Liste des prêts actifs et en retard

**Livrable** : Page prêts avec interface complète

### Étape 1.7: Dashboard avec statistiques fictives (1 jour)
**Objectif** : Page d'accueil avec métriques
- [ ] Composants `StatCard` pour métriques
- [ ] Calculs basés sur les données fictives
- [ ] Graphiques simples (optionnel)
- [ ] Liens vers les autres pages

**Livrable** : Dashboard attrayant avec statistiques

## Phase 2: Backend Minimal et Intégration

### Étape 2.1: Configuration backend de base (1 jour)
**Objectif** : Backend Axum minimal fonctionnel
- [ ] Créer structure backend/
- [ ] Configurer Cargo.toml backend (Axum + SQLx + PostgreSQL)
- [ ] Créer serveur Axum basique
- [ ] Endpoint `/health` simple
- [ ] Tester avec curl

**Livrable** : Serveur backend répondant sur http://localhost:8000

### Étape 2.2: Base de données PostgreSQL (1 jour)
**Objectif** : Connexion PostgreSQL fonctionnelle
- [ ] Configurer SQLx avec PostgreSQL
- [ ] Créer configuration de base de données
- [ ] Migration initiale pour table `titles`
- [ ] Endpoint `/health` vérifiant la DB

**Livrable** : Backend connecté à PostgreSQL

### Étape 2.3: API Titles basique (1 jour)
**Objectif** : CRUD titles côté backend
- [ ] Modèle `Title` backend
- [ ] Repository `TitleRepository`
- [ ] Endpoints `GET /api/v1/titles` et `POST /api/v1/titles`
- [ ] Tester avec curl/Postman

**Livrable** : API titles fonctionnelle

### Étape 2.4: Intégration frontend-backend Titles (1 jour)
**Objectif** : Remplacer données fictives par vraies données
- [ ] Service `ApiClient` côté frontend
- [ ] Modifier `TitlesPage` pour utiliser l'API
- [ ] Modifier `TitleForm` pour poster vers l'API
- [ ] Gestion des erreurs HTTP

**Livrable** : Frontend connecté au backend pour les titres

### Étape 2.5: Table Volumes et relation (1 jour)
**Objectif** : Gestion des volumes physiques
- [ ] Migration PostgreSQL pour table `volumes`
- [ ] Modèle `Volume` backend avec relation vers `Title`
- [ ] Repository `VolumeRepository`
- [ ] Endpoints volumes de base

**Livrable** : API volumes fonctionnelle

### Étape 2.6: Intégration Volumes frontend (1 jour)
**Objectif** : Afficher volumes par titre
- [ ] Modifier `TitleCard` pour afficher volumes
- [ ] Page détail titre avec liste volumes
- [ ] Formulaire ajout volume
- [ ] Intégration avec backend

**Livrable** : Gestion complète titre/volumes

### Étape 2.7: Génération codes-barres (1 jour)
**Objectif** : Codes-barres automatiques
- [ ] Service `BarcodeGenerator` backend
- [ ] Génération séquentielle (VOL-000001)
- [ ] Modification création volume
- [ ] Affichage codes-barres frontend

**Livrable** : Volumes avec codes-barres automatiques

### Étape 2.8: Scanner fonctionnel (1 jour)
**Objectif** : Recherche par code-barres réelle
- [ ] Endpoint `GET /api/v1/scan/volume/{barcode}`
- [ ] Intégration avec `ScanPage`
- [ ] Remplacer simulation par vraie recherche
- [ ] Gestion cas "non trouvé"

**Livrable** : Scanner fonctionnel avec backend

## Phase 2: Volume Management

### Step 2.1: Volumes table PostgreSQL (1 day)
**Objective**: Structure for physical volumes
- [ ] Create PostgreSQL migration for `volumes` table
- [ ] `Volume` model with relation to `Title`
- [ ] `VolumeRepository` with basic CRUD
- [ ] PostgreSQL foreign key constraints

**Deliverable**: Volumes table with relation to titles

### Step 2.2: Barcode generation (1 day)
**Objective**: Each volume has a unique code
- [ ] Create `BarcodeGenerator` service
- [ ] Implement sequential generation (VOL-000001)
- [ ] Modify volume creation to generate code
- [ ] Display barcode in interface

**Deliverable**: Volumes created with automatic barcodes

### Step 2.3: Basic volumes API (1 day)
**Objective**: CRUD for volumes
- [ ] Endpoints `POST /api/v1/titles/{id}/volumes`
- [ ] Endpoint `GET /api/v1/volumes/{id}`
- [ ] Endpoint `PUT /api/v1/volumes/{id}`
- [ ] Volume data validation

**Deliverable**: Functional volumes API

### Step 2.4: Volumes interface Leptos (1-2 days)
**Objective**: Manage volumes from interface
- [ ] `VolumeCard` component with volume details
- [ ] "Add volume" button on title page
- [ ] `AddVolumeForm` form
- [ ] Display volumes list by title

**Deliverable**: Complete volume management interface

### Step 2.5: Barcode search (1 day)
**Objective**: Find a volume by its code
- [ ] Implement `find_by_barcode` in repository
- [ ] Create endpoint `GET /api/v1/scan/volume/{barcode}`
- [ ] Add `scan_volume()` to frontend API service
- [ ] Test barcode search

**Deliverable**: Functional barcode search API

### Step 2.6: Leptos scan page (1-2 days)
**Objective**: Interface to scan/enter a barcode
- [ ] Create `ScanPage` with Leptos routing
- [ ] Reactive input field with validation
- [ ] Use create_action for search
- [ ] Conditional display of found volume
- [ ] Elegant handling of "volume not found" case

**Example code**:
```rust
#[component]
pub fn ScanPage() -> impl IntoView {
    let (barcode, set_barcode) = create_signal(String::new());
    let scan_action = create_action(|barcode: &String| {
        let barcode = barcode.clone();
        async move { api_client().scan_volume(&barcode).await }
    });

    view! {
        <div class="max-w-2xl mx-auto">
            <h1>"Scan a barcode"</h1>
            <input 
                prop:value=barcode
                on:input=move |ev| set_barcode(event_target_value(&ev))
                on:keydown=move |ev| {
                    if ev.key() == "Enter" && !barcode.get().is_empty() {
                        scan_action.dispatch(barcode.get());
                    }
                }
                placeholder="Scan or enter barcode..."
                autofocus
            />
            
            <Show when=move || scan_action.pending().get()>
                <p>"Searching..."</p>
            </Show>
            
            {move || scan_action.value().get().map(|result| match result {
                Ok(Some(volume)) => view! {
                    <div class="mt-4 p-4 bg-green-50 border border-green-200 rounded">
                        <h3>"Volume found!"</h3>
                        <VolumeCard volume=volume />
                    </div>
                }.into_view(),
                Ok(None) => view! {
                    <p class="text-orange-600">"No volume found"</p>
                }.into_view(),
                Err(_) => view! {
                    <p class="text-red-600">"Search error"</p>
                }.into_view(),
            })}
        </div>
    }
}
```

**Deliverable**: Functional Leptos scan page

## Phase 3: Simple Loan System

### Step 3.1: Borrowers table PostgreSQL (1 day)
**Objective**: Be able to register borrowers
- [ ] Create PostgreSQL migration for `borrowers` table
- [ ] Simple `Borrower` model (name + email)
- [ ] `BorrowerRepository` optimized for PostgreSQL
- [ ] Basic CRUD API for borrowers

**Deliverable**: Basic borrower management in PostgreSQL

### Step 3.2: Loans table PostgreSQL (1 day)
**Objective**: Record loans
- [ ] Create PostgreSQL migration for `loans` table
- [ ] `Loan` model (title_id, volume_id, borrower_id, dates)
- [ ] `LoanRepository` with optimized PostgreSQL queries
- [ ] PostgreSQL constraints and indexes

**Deliverable**: Data structure for loans in PostgreSQL

### Step 3.3: Simple loan API (1-2 days)
**Objective**: Loan a volume
- [ ] Endpoint `POST /api/v1/loans`
- [ ] Check that volume is available
- [ ] Create loan with due date
- [ ] Mark volume as loaned
- [ ] Automatic best volume selection logic

**Deliverable**: Functional loan API

### Step 3.4: Return API (1 day)
**Objective**: Return a loaned volume
- [ ] Endpoint `PUT /api/v1/loans/{id}/return`
- [ ] Mark loan as returned
- [ ] Free the volume
- [ ] Business error handling

**Deliverable**: Functional return API

### Step 3.5: Leptos loan/return interface (2 days)
**Objective**: Loan/return from interface
- [ ] `BorrowerSelector` component with reactive search
- [ ] Add loan/return buttons on scan page
- [ ] Leptos modal for borrower selection
- [ ] Reactive actions for loan/return
- [ ] Toast notifications with signals

**Example code**:
```rust
#[component]
pub fn LoanActions(volume: Volume) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let loan_action = create_action(|req: &LoanRequest| {
        let req = req.clone();
        async move { api_client().create_loan(req).await }
    });

    view! {
        <div class="flex space-x-2">
            <button 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                on:click=move |_| set_show_modal(true)
            >
                "Loan"
            </button>
            
            <Show when=show_modal>
                <BorrowerModal 
                    volume=volume.clone()
                    on_close=move || set_show_modal(false)
                    on_loan=move |borrower_id| {
                        loan_action.dispatch(LoanRequest {
                            title_id: volume.title_id,
                            borrower_id,
                        });
                        set_show_modal(false);
                    }
                />
            </Show>
        </div>
    }
}
```

**Deliverable**: Complete Leptos loan/return interface

## Phase 4: Améliorations et Fonctionnalités Avancées

### Étape 4.1: États des volumes (1 jour)
**Objectif** : Gérer l'état physique des volumes
- [ ] Ajouter champ `condition` ENUM à la table volumes PostgreSQL
- [ ] Enum Rust pour états (excellent, bon, correct, mauvais, endommagé)
- [ ] Composant Leptos `ConditionSelector`
- [ ] Empêcher prêt des volumes en mauvais état

**Livrable** : Gestion des états de volume

### Étape 4.2: Informations étendues des titres (1-2 jours)
**Objectif** : Plus de métadonnées
- [ ] Ajouter champs PostgreSQL : éditeur, année, pages, genre
- [ ] Modifier migrations PostgreSQL et modèles Rust
- [ ] Formulaire `TitleForm` étendu avec validation réactive
- [ ] Améliorer `TitleCard` avec toutes les infos

**Livrable** : Titres avec métadonnées complètes

### Étape 4.3: Localisation physique (1 jour)
**Objectif** : Savoir où sont stockés les volumes
- [ ] Ajouter champ `location` simple
- [ ] Composant `LocationInput` avec suggestions
- [ ] Afficher localisation dans listes
- [ ] Filtrer par localisation

**Livrable** : Gestion de localisation physique

### Étape 4.4: Gestion auteurs (2 jours)
**Objectif** : Associer auteurs aux titres
- [ ] Créer table PostgreSQL `authors`
- [ ] Table de jonction `title_authors`
- [ ] API CRUD auteurs
- [ ] Composants Leptos `AuthorsList` et `AuthorForm`

**Livrable** : Gestion de base des auteurs

### Étape 4.5: Association titre-auteur Leptos (1-2 jours)
**Objectif** : Lier titres et auteurs
- [ ] Composant `AuthorSelector` avec recherche temps réel
- [ ] Modifier `TitleForm` pour sélectionner auteurs
- [ ] Afficher auteurs dans `TitleCard`
- [ ] Page détail auteur avec ses titres

**Livrable** : Liaison fonctionnelle titre-auteur

## Phase 5: Fonctionnalités Avancées

### Étape 5.1: Recherche textuelle (1-2 jours)
**Objectif** : Recherche dans titres et descriptions
- [ ] Ajouter index FULLTEXT PostgreSQL
- [ ] Endpoint recherche avec requêtes PostgreSQL
- [ ] Composant `SearchBar` avec debouncing
- [ ] Filtres réactifs par disponibilité, état, etc.

**Livrable** : Recherche avancée fonctionnelle

### Étape 5.2: Classification Dewey (2 jours)
**Objectif** : Classer les titres selon Dewey
- [ ] Ajouter champs `dewey_code` et `dewey_category` en PostgreSQL
- [ ] Table de référence PostgreSQL pour codes Dewey
- [ ] Composant `DeweySelector` avec navigation hiérarchique
- [ ] Navigation par classification

**Livrable** : Classification Dewey de base

### Étape 5.3: Gestion séries (2-3 jours)
**Objectif** : Organiser les titres en séries
- [ ] Créer table PostgreSQL `series`
- [ ] Associer titres aux séries
- [ ] Composant `SeriesManager`
- [ ] Interface gestion séries

**Livrable** : Gestion fonctionnelle des séries

### Étape 5.4: Gestion des retards (1-2 jours)
**Objectif** : Identifier et gérer les prêts en retard
- [ ] Calcul automatique des retards
- [ ] Endpoint `/api/v1/loans/overdue`
- [ ] Composant `OverdueLoans` avec actions
- [ ] Notifications visuelles réactives

**Livrable** : Gestion des prêts en retard

## Phase 6: Support Multi-SGBD et Internationalisation

### Étape 6.1: Abstraction base de données multi-SGBD (2-3 jours)
**Objectif** : Support PostgreSQL (défaut), MySQL, MariaDB
- [ ] Créer `DatabaseFactory` avec PostgreSQL par défaut
- [ ] Migrations séparées : PostgreSQL (principal), MySQL, MariaDB
- [ ] Configuration dynamique type DB
- [ ] Tests avec les 3 SGBD

**Livrable** : Support multi-SGBD

### Étape 6.2: Internationalisation backend (2 jours)
**Objectif** : Messages d'erreur multilingues
- [ ] Service `I18nService`
- [ ] Fichiers de traduction (FR, EN)
- [ ] Middleware détection langue
- [ ] Messages API traduits

**Livrable** : Backend multilingue

### Étape 6.3: Internationalisation Leptos (2-3 jours)
**Objectif** : Interface utilisateur multilingue
- [ ] Intégration leptos-i18n
- [ ] Composant `LanguageSelector`
- [ ] Traduction de tous les textes avec signaux réactifs
- [ ] Formats de date localisés

**Livrable** : Frontend Leptos complètement multilingue

## Phase 7: Fonctionnalités Avancées

### Étape 7.1: Import/Export CSV (2 jours)
**Objectif** : Importer/exporter des données
- [ ] Endpoint export CSV
- [ ] Endpoint import CSV avec validation
- [ ] Composant Leptos `ImportExport` avec upload fichier
- [ ] Gestion erreurs import avec feedback

**Livrable** : Import/export de données

### Étape 7.2: Génération d'étiquettes (2-3 jours)
**Objectif** : Imprimer étiquettes avec codes-barres
- [ ] Génération images codes-barres
- [ ] Templates étiquettes (Avery, etc.)
- [ ] Endpoint génération PDF
- [ ] Interface Leptos sélection et prévisualisation

**Livrable** : Génération d'étiquettes imprimables

### Étape 7.3: Intégration API externe (2-3 jours)
**Objectif** : Récupérer métadonnées via ISBN
- [ ] Client API Google Books
- [ ] Récupération automatique métadonnées
- [ ] Téléchargement couvertures
- [ ] Composant Leptos `ISBNImport` avec prévisualisation

**Livrable** : Enrichissement automatique des données

### Étape 7.4: Notifications et rappels (2 jours)
**Objectif** : Alertes de prêts
- [ ] Service de notification
- [ ] Calcul dates d'échéance
- [ ] Composant Leptos `NotificationCenter`
- [ ] Notifications toast réactives

**Livrable** : Système de notifications

## Phase 8: Optimisation et Déploiement

### Étape 8.1: Cache Redis (1-2 jours)
**Objectif** : Améliorer les performances
- [ ] Intégration Redis
- [ ] Cache des requêtes fréquentes
- [ ] Invalidation intelligente
- [ ] Métriques de cache

**Livrable** : Système de cache performant

### Étape 8.2: Tests automatisés (2-3 jours)
**Objectif** : Couverture de tests complète
- [ ] Tests unitaires pour tous les services
- [ ] Tests d'intégration API avec base PostgreSQL de test
- [ ] Tests Leptos avec wasm-bindgen-test
- [ ] CI/CD avec GitHub Actions

**Livrable** : Suite de tests complète

### Étape 8.3: Conteneurisation (1-2 jours)
**Objectif** : Déploiement Docker
- [ ] Dockerfiles optimisés (multi-stage pour Leptos)
- [ ] Docker Compose avec PostgreSQL par défaut
- [ ] Scripts de déploiement
- [ ] Documentation déploiement

**Livrable** : Déploiement conteneurisé

### Étape 8.4: Monitoring et logs (1-2 jours)
**Objectif** : Observabilité en production
- [ ] Logs structurés avec tracing
- [ ] Métriques de performance
- [ ] Checks de santé
- [ ] Alertes de monitoring

**Livrable** : Monitoring complet

## Spécifications Techniques

### Avantages Leptos pour ce projet :
- **Réactivité fine** : Les listes de titres se mettent à jour automatiquement
- **Sécurité de type** : Partage de types entre frontend et backend
- **Performance** : Compilation WASM optimisée
- **Composants réutilisables** : `TitleCard`, `SearchBar`, etc.

### Avantages PostgreSQL pour ce projet :
- **Performance** : Requêtes optimisées pour recherches textuelles
- **FULLTEXT** : Recherche textuelle native performante
- **JSON** : Support natif colonnes JSON pour métadonnées
- **Fiabilité** : ACID complet, contraintes robustes
- **Extensibilité** : Extensions pour fonctionnalités avancées

### Outils recommandés :
- **Trunk** : Outil de build pour WASM
- **leptos-i18n** : Internationalisation
- **reqwest-wasm** : Client HTTP
- **web-sys** : Interactions DOM
- **wasm-bindgen** : Bindings JavaScript
- **SQLx** : Requêtes PostgreSQL type-safe

## Estimation Totale
- **Phase 1** : 7-10 jours (Frontend avec données fictives)
- **Phase 2** : 8-12 jours (Backend minimal et intégration)
- **Phase 3** : 6-8 jours (Système prêts)
- **Phase 4** : 8-12 jours (Améliorations)
- **Phase 5** : 8-12 jours (Fonctionnalités avancées)
- **Phase 6** : 6-8 jours (Multi-SGBD + i18n)
- **Phase 7** : 8-12 jours (Fonctionnalités avancées)
- **Phase 8** : 6-9 jours (Optimisation)

**Total estimé** : 57-83 jours de développement

## Notes Importantes
- Chaque étape doit être testée avant de passer à la suivante
- Les estimations incluent tests et documentation
- Leptos permet un développement plus rapide grâce à la réactivité
- Prévoir du temps d'apprentissage Leptos si nécessaire
- Chaque étape produit une version déployable et utilisable
- La réactivité Leptos simplifie la gestion d'état complexe
- PostgreSQL par défaut avec support multi-SGBD pour flexibilité
- **Approche frontend-first** : Interface utilisable dès la Phase 1, backend ajouté progressivement
