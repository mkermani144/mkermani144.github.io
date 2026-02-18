use chrono::Local;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{FlatRoutes, Route, Router},
    hooks::use_params_map,
    path,
    static_routes::{StaticParamsMap, StaticRoute},
};

use crate::{blog, components::layout::Layout};

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
                        ssr=SsrMode::Static(
                            StaticRoute::new().prerender_params(|| async {
                                let slugs = blog::all_slugs()
                                    .into_iter()
                                    .map(|slug| slug.to_string())
                                    .collect::<Vec<_>>();
                                let mut params = StaticParamsMap::new();
                                params.insert("slug", slugs);
                                params
                            }),
                        )
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
    let last_updated = Local::now().format("%B %-d, %Y at %-I:%M %p").to_string();

    view! {
        <section class="text-start">
            <h1 class="text-3xl font-semibold tracking-tight pb-8 mb-8 border-b-4 border-double border-black w-fit">"Mohammad Kermani"</h1>
            <p class="mt-2">
                "I'm a husband, a child, a friend, a software engineer, a lifelong learner, an adventurer, a mentor, and maybe someday, an entrepreneur."
            </p>
            <p class="mt-2">
                "I like nature, books, people, tinkering, traveling, and sometimes household chores."
            </p>
            <p class="mt-8">
                <a class="font-medium text-primary underline underline-offset-4 hover:text-primary/85" href="/blog.html">
                    "Read my blog"
                </a>
                " or check some of my profiles:"
            </p>

            <ul class="mt-4 flex flex-wrap items-center justify-start gap-5 text-stone-600">
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

            <p class="mt-6 text-xs text-stone-500">{format!("Last updated: {last_updated}")}</p>

        </section>
    }
}

#[component]
fn BlogIndexPage() -> impl IntoView {
    let mut posts = blog::all_posts().to_vec();
    posts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    view! {
        <section>
            <h1 class="text-2xl font-semibold tracking-tight">"None of the 404 blogs was found."</h1>
            <h1 class="text-l tracking-tight">"It's planned, stay tuned."</h1>
            <ul class="mt-5 space-y-3">
                {posts
                    .into_iter()
                    .map(|post| {
                        view! {
                            <li>
                                <a
                                    class="text-lg font-medium text-primary underline underline-offset-4 hover:text-primary/85"
                                    href=format!("/blog/{}.html", post.slug)
                                >
                                    {post.title.to_string()}
                                </a>
                                <p class="mt-1 text-sm text-stone-500">{post.date.to_string()}</p>
                                <p class="mt-1 text-stone-600">{post.summary.to_string()}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        {move || {
            let slug = params.read().get("slug").unwrap_or_default();
            match blog::by_slug(&slug) {
                Some(post) => {
                    view! {
                        <section>
                            <p class="text-sm">
                                <a class="text-primary underline underline-offset-4 hover:text-primary/85" href="/blog.html">
                                    "← Back to blog"
                                </a>
                            </p>
                            <h1 class="mt-4 text-4xl font-semibold tracking-tight">{post.title.to_string()}</h1>
                            <p class="mt-2 text-sm text-stone-500">
                                {format!("Published {}", post.date)}
                            </p>
                            <article class="markdown-content mt-5" inner_html=post.html.to_string()></article>
                        </section>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <section>
                            <h1 class="text-2xl font-semibold tracking-tight">"Post not found"</h1>
                            <p class="mt-3 text-stone-600">"The requested post does not exist."</p>
                            <p class="mt-6">
                                <a class="text-primary underline underline-offset-4 hover:text-primary/85" href="/blog.html">
                                    "← Back to blog"
                                </a>
                            </p>
                        </section>
                    }
                        .into_any()
                }
            }
        }}
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
