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
    let last_updated = Local::now().format("%B %-d, %Y").to_string();

    view! {
        <section class="text-start">
            <p class="kicker">"Welcome to my home, stranger 👋"</p>
            <h1 class="mt-3 text-3xl font-semibold tracking-tight text-stone-900 sm:text-4xl">
                "Hi, I'm Mohammad"
            </h1>
            <p class="section-intro mt-5 max-w-3xl">
                "I'm a husband, a child, a friend, a software engineer, a lifelong learner, an adventurer, a mentor, and maybe someday, an entrepreneur."
            </p>
            <p class="section-intro max-w-3xl">
                "I like nature, books, people, tinkering, traveling, and sometimes household chores."
            </p>
            <p class="mt-7 section-intro">
                <a class="font-medium text-primary  underline-offset-4 hover:text-primary/85" href="/blog.html">
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

            <p class="mono-text mt-6 text-xs text-stone-500">{format!("Last updated: {last_updated}")}</p>
        </section>
    }
}

#[component]
fn BlogIndexPage() -> impl IntoView {
    let mut posts = blog::all_posts().to_vec();
    posts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    view! {
        <section class="text-start">
            <p class="kicker">"Post-introspection thoughts - Digest with a grain of salt"</p>
            <h1 class="mt-3 text-3xl font-semibold tracking-tight text-stone-900 sm:text-4xl">"Blog"</h1>

            {if posts.is_empty() {
                view! {
                    <section class="section-shell">
                        <p class="section-intro">"None of the 404 blogs was found."</p>
                    </section>
                }
                    .into_any()
            } else {
                view! {
                    <ul class="line-list mt-6">
                        {posts
                            .into_iter()
                            .map(|post| {
                                view! {
                                    <li class="line-item">
                                        <a
                                            class="text-xl font-medium text-primary underline underline-offset-4 hover:text-primary/85"
                                            href=format!("/blog/{}.html", post.slug)
                                        >
                                            {post.title.to_string()}
                                        </a>
                                        <p class="mono-text mt-2 text-xs text-stone-500">{post.date.to_string()}</p>
                                        <p class="mt-2 text-stone-700">{post.summary.to_string()}</p>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                }
                    .into_any()
            }}
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
                            <p class="mono-text text-xs text-stone-500">
                                <a class="text-primary underline underline-offset-4 hover:text-primary/85" href="/blog.html">
                                    "← Back to blog"
                                </a>
                            </p>
                            <h1 class="mt-4 text-4xl font-semibold tracking-tight text-stone-900">{post.title.to_string()}</h1>
                            <p class="mono-text mt-2 text-xs text-stone-500">
                                {format!("Published {}", post.date)}
                            </p>
                            <article class="markdown-content mt-6" inner_html=post.html.to_string()></article>
                        </section>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <section class="text-start">
                            <h1 class="text-2xl font-semibold tracking-tight text-stone-900">"Post not found"</h1>
                            <p class="mt-3 text-stone-600">"The requested post does not exist."</p>
                            <p class="mt-6 mono-text text-xs text-stone-500">
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
            <p class="kicker text-primary">"404"</p>
            <h1 class="mt-2 text-3xl font-semibold tracking-tight text-stone-900">"Page not found"</h1>
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
