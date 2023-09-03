use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages;
use pages::home::HomePage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/q3aparser.css"/>

        // sets the document title
        <Title text="Quake 3 Arena server log parser"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main class="w-screen h-full min-h-screen pt-12 bg-[#262f3d] text-[#e8ecf0]">
                <div class="mx-auto max-w-[1080px]">
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
