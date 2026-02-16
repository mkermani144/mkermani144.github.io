use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-stone-100 text-stone-800">
            <div class="mx-auto w-full max-w-3xl px-6 py-16">
                <header class="mb-10 flex items-center justify-start gap-6 text-sm font-medium text-stone-600">
                    <a class="underline decoration-stone-400 underline-offset-4 hover:text-primary" href="/">
                        "Home"
                    </a>
                    <a class="underline decoration-stone-400 underline-offset-4 hover:text-primary" href="/blog.html">
                        "Blog"
                    </a>
                </header>
                <main class="min-h-80 grid place-items-center">
                    <div class="w-full">{children()}</div>
                </main>
            </div>
        </div>
    }
}
