use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use leptonic::prelude::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (text, set_text) = create_signal(String::new());
    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic SSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#8856e6"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Leptonic SSR template"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
            }>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                </Routes>
            </Router>
        </Root>    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <h1>Hello</h1>
    }
}
