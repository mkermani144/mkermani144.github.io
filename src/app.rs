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
                <FlatRoutes fallback=|| view! { <NotFoundPage/> }>
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
                    <Route
                        path=path!("/404")
                        view=NotFoundPage
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
        <section class="text-center">
            <h1 class="text-3xl font-semibold tracking-tight">"Mohammad Kermani"</h1>
            <p class="mt-4 text-stone-600">
                "Software Engineer"
            </p>
            <p class="mt-2 text-stone-500">
                "I love building things that people use."
            </p>

            <ul class="mt-8 flex flex-wrap items-center justify-center gap-5 text-stone-600">
                <li>
                    <a
                        class="underline decoration-stone-400 underline-offset-4 hover:text-primary"
                        href="https://github.com/mkermani144"
                        rel="noreferrer"
                        target="_blank"
                    >
                        "GitHub"
                    </a>
                </li>
                <li>
                    <a
                        class="underline decoration-stone-400 underline-offset-4 hover:text-primary"
                        href="https://linkedin.com/in/mkermani144"
                        rel="noreferrer"
                        target="_blank"
                    >
                        "LinkedIn"
                    </a>
                </li>
                <li>
                    <a
                        class="underline decoration-stone-400 underline-offset-4 hover:text-primary"
                        href="https://x.com/mkermani144"
                        rel="noreferrer"
                        target="_blank"
                    >
                        "X"
                    </a>
                </li>
            </ul>

            <p class="mt-8">
                <a class="font-medium text-primary underline underline-offset-4 hover:text-primary/85" href="/blog.html">
                    "Read the blog"
                </a>
            </p>
        </section>
    }
}

#[component]
fn BlogIndexPage() -> impl IntoView {
    view! {
        <section>
            <h1 class="text-2xl font-semibold tracking-tight">"Blog"</h1>
            <p class="mt-3 text-stone-600">"Posts will be listed here."</p>
        </section>
    }
}

#[component]
fn BlogPostPage() -> impl IntoView {
    view! {
        <section>
            <h1 class="text-2xl font-semibold tracking-tight">"Post"</h1>
            <p class="mt-3 text-stone-600">"Post content will render here."</p>
        </section>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="text-center">
            <p class="text-sm uppercase tracking-widest text-primary">"404"</p>
            <h1 class="mt-2 text-3xl font-semibold tracking-tight">"Page not found"</h1>
            <p class="mt-3 text-stone-600">
                "The page you requested does not exist."
            </p>
            <p class="mt-8">
                <a class="font-medium text-primary underline underline-offset-4 hover:text-primary/85" href="/">
                    "Go home"
                </a>
            </p>
        </section>
    }
}
