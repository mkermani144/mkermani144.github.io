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
                <script>{r#"
                    document.addEventListener('click', function(e) {
                        var img = e.target.closest('.forge-img');
                        if (!img) return;
                        var m = document.createElement('div');
                        m.className = 'forge-modal';
                        var i = document.createElement('img');
                        i.src = img.src;
                        i.alt = img.alt;
                        m.appendChild(i);
                        document.body.appendChild(m);
                        requestAnimationFrame(function() { m.setAttribute('data-open', ''); });
                        m.addEventListener('click', function() {
                            m.removeAttribute('data-open');
                            setTimeout(function() { m.remove(); }, 200);
                        });
                    });
                    document.addEventListener('keydown', function(e) {
                        if (e.key === 'Escape') {
                            var m = document.querySelector('.forge-modal');
                            if (m) { m.removeAttribute('data-open'); setTimeout(function() { m.remove(); }, 200); }
                        }
                    });
                "#}</script>
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
                        path=path!("/forge")
                        view=ForgePage
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
fn ForgeCardHeader(
    title: &'static str,
    date: &'static str,
    #[prop(optional)] role: &'static str,
    #[prop(optional)] company: &'static str,
    #[prop(optional)] github: &'static str,
    #[prop(optional)] website: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-baseline justify-between gap-2">
            <div class="flex items-center gap-2">
                <h2 class="forge-card-title">{title}</h2>
                {(!github.is_empty()).then(|| view! {
                    <a class="forge-card-icon" href=github target="_blank" rel="noreferrer" aria-label="GitHub">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
                            <path d="M9 18c-4.51 2-5-2-7-2"/>
                        </svg>
                    </a>
                })}
                {(!website.is_empty()).then(|| view! {
                    <a class="forge-card-icon" href=website target="_blank" rel="noreferrer" aria-label="Website">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
                            <polyline points="15 3 21 3 21 9"/>
                            <line x1="10" y1="14" x2="21" y2="3"/>
                        </svg>
                    </a>
                })}
            </div>
            <span class="forge-card-date">{date}</span>
        </div>
        {(!role.is_empty() || !company.is_empty()).then(|| view! {
            <p class="forge-card-meta">
                {(!role.is_empty()).then(|| view! { <span class="forge-card-role">{role}</span> })}
                {(!role.is_empty() && !company.is_empty()).then(|| view! { <span class="forge-card-meta-sep">" · "</span> })}
                {(!company.is_empty()).then(|| view! { <span class="forge-card-company">{company}</span> })}
            </p>
        })}
    }
}

#[component]
fn ForgePage() -> impl IntoView {
    view! {
        <section class="text-start">
            <p class="kicker">"Take a seat. I'll show you what I built, shipped, and measured."</p>
            <h1 class="mt-3 text-3xl font-semibold tracking-tight text-stone-900 sm:text-4xl">"Forge"</h1>

            <div class="mt-8 grid grid-cols-2 gap-3 sm:grid-cols-4">
                <div class="forge-hero-stat">
                    <p class="forge-hero-number">"8+"</p>
                    <p class="forge-hero-label">"years"</p>
                </div>
                <div class="forge-hero-stat">
                    <p class="forge-hero-number">"4"</p>
                    <p class="forge-hero-label">"startups"</p>
                </div>
                <div class="forge-hero-stat">
                    <p class="forge-hero-number">"200K+"</p>
                    <p class="forge-hero-label">"users reached"</p>
                </div>
                <div class="forge-hero-stat">
                    <p class="forge-hero-number">"10+"</p>
                    <p class="forge-hero-label">"engineers led"</p>
                </div>
            </div>

            <div class="forge-grid">

                // ── EpisAI group ──
                <div class="forge-group">
                    <div class="forge-group-header">
                        <span class="forge-group-company">"EpisAI"</span>
                        <span class="forge-group-sep">"·"</span>
                        <span class="forge-group-role">"Founder"</span>
                        <span class="forge-group-date">"2025"</span>
                    </div>
                    <div class="forge-group-grid">
                        <div class="forge-card forge-card--half">
                            <ForgeCardHeader title="Epis" date="" github="https://github.com/mkermani144/epis"/>
                            <p class="forge-card-desc">"Voice-based language learning AI app with real-time AI conversations."</p>
                            <div class="forge-highlights">
                                <div class="forge-highlight">"Mnemonic generation"</div>
                                <div class="forge-highlight">"CEFR level perception & adaptation"</div>
                                <div class="forge-highlight">"Spaced interleaving (1d, 2d, 4d, 8d…)"</div>
                                <div class="forge-highlight">"Multi-language support"</div>
                            </div>
                            <div class="forge-tags">
                                {["Rust", "Axum", "OpenAI", "WebSockets", "SQLx", "DDD"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                        <div class="forge-card forge-card--half">
                            <ForgeCardHeader title="Jessy" date="" github="https://github.com/mkermani144/jessy"/>
                            <p class="forge-card-desc">"Job search AI assistant — async pipeline that crawls, filters, and matches positions."</p>
                            <div class="forge-before-after">
                                <div class="forge-ba-item">
                                    <span class="forge-ba-label">"Before"</span>
                                    <div class="forge-ba-bar forge-ba-bar--before" style="width: 100%"></div>
                                    <span class="forge-ba-value">"+3h/day"</span>
                                </div>
                                <div class="forge-ba-item">
                                    <span class="forge-ba-label">"After"</span>
                                    <div class="forge-ba-bar forge-ba-bar--after" style="width: 8%"></div>
                                    <span class="forge-ba-value">"15min/day"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["Rust", "OpenAI", "RAG", "PostgreSQL", "Docker"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                    </div>
                </div>

                // ── Pattern (full, standalone) ──
                <div class="forge-card forge-card--full">
                    <ForgeCardHeader title="Pattern" date="2025" role="Founding Engineer" website="https://pattern.global" github="https://github.com/pattern-tech"/>
                    <p class="forge-card-desc">"AI chat assistant over blockchain data — RAG, tool calling, and wallet-based auth from day one."</p>
                    <div class="forge-gallery">
                        <img class="forge-img" src="/forge/pattern-chat.webp" alt="Pattern AI chat interface"/>
                        <img class="forge-img" src="/forge/pattern-activity.jpg" alt="Pattern wallet activity query"/>
                    </div>
                    <div class="forge-tags">
                        {["FastAPI", "Next.js", "Vercel AI SDK", "PostgreSQL", "Nginx", "AWS"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

                // ── Ergo Platform group ──
                <div class="forge-group">
                    <div class="forge-group-header">
                        <span class="forge-group-company">"Rosen Bridge, Ergo Platform"</span>
                        <span class="forge-group-sep">"·"</span>
                        <span class="forge-group-role">"Software Engineer"</span>
                        <span class="forge-group-date">"2022 – 2024"</span>
                    </div>
                    <div class="forge-group-grid">
                        <div class="forge-card forge-card--full">
                            <ForgeCardHeader title="Rosen Bridge" date="" github="https://github.com/rosen-bridge" website="https://rosen.tech"/>
                            <p class="forge-card-desc">"Cross-chain crypto bridge connecting Ethereum, Bitcoin, Cardano, and more. Built 3 fullstack apps."</p>
                            <div class="forge-metrics">
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"$10M+"</span>
                                    <span class="forge-metric-label">"escrowed"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"100K+"</span>
                                    <span class="forge-metric-label">"events tracked"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"~10"</span>
                                    <span class="forge-metric-label">"chains"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"~400"</span>
                                    <span class="forge-metric-label">"participants"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["TypeScript", "Next.js", "Rust", "PostgreSQL", "Redis", "Docker", "Vercel"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                        <div class="forge-card forge-card--full">
                            <ForgeCardHeader title="RoseNet" date="" github="https://github.com/rosen-bridge/rosenet"/>
                            <p class="forge-card-desc">"P2P networking layer with a custom protocol on top of TCP — designed and validated through sustained load and stress testing."</p>
                            <div class="forge-metrics">
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"98%"</span>
                                    <span class="forge-metric-label">"message delivery"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"200Mb/s"</span>
                                    <span class="forge-metric-label">"sustained 24h"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"~600ms"</span>
                                    <span class="forge-metric-label">"p99 latency"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"90%"</span>
                                    <span class="forge-metric-label">"mem reduction"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["Rust", "TypeScript", "Libp2p", "TCP", "Prometheus", "Grafana"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                        <div class="forge-card forge-card--third">
                            <ForgeCardHeader title="Kodegen" date="" github="https://github.com/rosen-bridge/kodegen"/>
                            <p class="forge-card-desc">"Code generation toolkit — boilerplate setup enforcing standards across teams."</p>
                            <div class="forge-metrics">
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"30+"</span>
                                    <span class="forge-metric-label">"packages"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"~15"</span>
                                    <span class="forge-metric-label">"feature flags"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["TypeScript", "Node.js"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                        <div class="forge-card forge-card--third">
                            <ForgeCardHeader title="Minimum Fee" date="" github="https://github.com/rosen-bridge/minimum-fee-job"/>
                            <p class="forge-card-desc">"Automated fee validation for bridge transactions."</p>
                            <div class="forge-before-after">
                                <div class="forge-ba-item">
                                    <span class="forge-ba-label">"Before"</span>
                                    <div class="forge-ba-bar forge-ba-bar--before" style="width: 100%"></div>
                                    <span class="forge-ba-value">"hours"</span>
                                </div>
                                <div class="forge-ba-item">
                                    <span class="forge-ba-label">"After"</span>
                                    <div class="forge-ba-bar forge-ba-bar--after" style="width: 5%"></div>
                                    <span class="forge-ba-value">"minutes"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["Next.js", "Redis", "Node.js"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                        <div class="forge-card forge-card--third">
                            <ForgeCardHeader title="Scanner" date="" github="https://github.com/rosen-bridge/scanner"/>
                            <p class="forge-card-desc">"Blockchain indexer scanning and storing on-chain events."</p>
                            <div class="forge-metrics">
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"10M+"</span>
                                    <span class="forge-metric-label">"blocks scanned"</span>
                                </div>
                                <div class="forge-metric">
                                    <span class="forge-metric-value">"~40K"</span>
                                    <span class="forge-metric-label">"blocks / day"</span>
                                </div>
                            </div>
                            <div class="forge-tags">
                                {["Node.js", "PostgreSQL", "SQLite"]
                                    .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                            </div>
                        </div>
                    </div>
                </div>

                // ── Balonet (full, standalone) ──
                <div class="forge-card forge-card--full">
                    <ForgeCardHeader title="Balonet" date="2018 – 2022" role="Founding Engineer / Tech Lead" company="Zamin"/>
                    <p class="forge-card-desc">"Team messenger with Notion-like features — desktop and web. Scaled to 200K active users."</p>
                    <div class="forge-gallery forge-gallery--3col">
                        <img class="forge-img" src="/forge/balonet-1.png" alt="Balonet screenshot 1"/>
                        <img class="forge-img" src="/forge/balonet-3.png" alt="Balonet screenshot 3"/>
                        <img class="forge-img" src="/forge/balonet-2.png" alt="Balonet screenshot 2"/>
                    </div>
                    <div class="forge-tags">
                        {["TypeScript", "React", "Electron", "Redux", "Vite", "Jest", "IndexedDB"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

                // ── id8r (half) ──
                <div class="forge-card forge-card--half">
                    <ForgeCardHeader title="ID8R" date="side project"/>
                    <p class="forge-card-desc">"NFT customization — pick popular NFTs, add frames, text, and overlays. Set as avatar or Twitter header."</p>
                    <div class="forge-gallery">
                        <img class="forge-img" src="/forge/id8r-product.webp" alt="ID8R product image"/>
                        <img class="forge-img" src="/forge/id8r-screenshot.webp" alt="ID8R app screenshot"/>
                    </div>
                    <div class="forge-tags">
                        {["Next.js", "MySQL", "AWS RDS", "Prisma"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

                // ── Arbit (half) ──
                <div class="forge-card forge-card--half">
                    <ForgeCardHeader title="Arbit" date="side project" github="https://github.com/ConnecMent/arbit"/>
                    <p class="forge-card-desc">"Cross-chain arbitrage detection via graph algorithms, with a Telegram bot that captured real profits."</p>
                    <div class="forge-metrics">
                        <div class="forge-metric">
                            <span class="forge-metric-value">"$5K+"</span>
                            <span class="forge-metric-label">"captured arbs"</span>
                        </div>
                        <div class="forge-metric">
                            <span class="forge-metric-value">"~50%"</span>
                            <span class="forge-metric-label">"CPU reduction"</span>
                        </div>
                    </div>
                    <div class="forge-tags">
                        {["TypeScript", "Vercel", "Telegram"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

                // ── Lodestar (half) ──
                <div class="forge-card forge-card--half">
                    <ForgeCardHeader title="Lodestar" date="open source" github="https://github.com/ChainSafe/lodestar"/>
                    <p class="forge-card-desc">"Contributed to the open-source Ethereum consensus client."</p>
                    <div class="forge-metrics">
                        <div class="forge-metric">
                            <span class="forge-metric-value">"3"</span>
                            <span class="forge-metric-label">"PRs merged"</span>
                        </div>
                        <div class="forge-metric">
                            <span class="forge-metric-value">"ETH"</span>
                            <span class="forge-metric-label">"consensus client"</span>
                        </div>
                    </div>
                    <div class="forge-tags">
                        {["TypeScript", "Ethereum"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

                // ── Wanna (half) ──
                <div class="forge-card forge-card--half">
                    <ForgeCardHeader title="Wanna" date="side project" github="https://github.com/mkermani144/wanna"/>
                    <p class="forge-card-desc">"A 21st-century to-do list app — desktop and web."</p>
                    <div class="forge-metrics">
                        <div class="forge-metric">
                            <span class="forge-metric-value">"~200"</span>
                            <span class="forge-metric-label">"GitHub stars"</span>
                        </div>
                    </div>
                    <div class="forge-tags">
                        {["React", "Electron", "Redux", "Material UI"]
                            .into_iter().map(|t| view! { <span class="forge-tag">{t}</span> }).collect_view()}
                    </div>
                </div>

            </div>
        </section>
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
