use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen px-4 py-8 text-stone-800 sm:px-6 sm:py-12">
            <div class="content-wrap mx-auto w-full max-w-4xl px-4 sm:px-6">
                <header class="mb-10 border-b border-stone-300/75 pb-5">
                    <nav class="flex flex-wrap items-center gap-5">
                        <a class="site-header-link" href="/">
                            "Home"
                        </a>
                        <a class="site-header-link forge-link" href="/forge.html">
                            "Forge"
                            <span class="forge-sparks" aria-hidden="true">
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                                <span class="forge-spark"></span>
                            </span>
                        </a>
                        <a class="site-header-link" href="/blog.html">
                            "Blog"
                        </a>
                    </nav>
                </header>
                <main class="min-h-[68vh]">
                    <div class="w-full">{children()}</div>
                </main>
            </div>
        </div>
    }
}
