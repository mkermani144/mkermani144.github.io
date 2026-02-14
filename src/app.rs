use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{FlatRoutes, Route, Router},
    path,
    static_routes::StaticRoute,
};

use crate::components::layout::Layout;

pub fn shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="site-style" href="/styles.css"/>
        <Router>
            <Layout>
                <FlatRoutes fallback=|| view! { <></> }>
                    <Route
                        path=path!("/")
                        view=HomePage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/blog")
                        view=BlogIndexPage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/blog/:slug")
                        view=BlogPostPage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                </FlatRoutes>
            </Layout>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section>
            <h1 class="text-3xl font-semibold tracking-tight">"Mohammad Kermani"</h1>
            <p class="mt-3 text-neutral-300">"I like to build things that people use."</p>
        </section>
    }
}

#[component]
fn BlogIndexPage() -> impl IntoView {
    view! {
        <section>
            <h1 class="text-2xl font-semibold tracking-tight">"Blog"</h1>
            <p class="mt-3 text-neutral-300">"Posts will be listed here."</p>
        </section>
    }
}

#[component]
fn BlogPostPage() -> impl IntoView {
    view! {
        <section>
            <h1 class="text-2xl font-semibold tracking-tight">"Post"</h1>
            <p class="mt-3 text-neutral-300">"Post content will render here."</p>
        </section>
    }
}
