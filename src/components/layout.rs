use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-stone-100 text-stone-800">
            <div class="mx-auto w-full max-w-3xl px-6 py-16">
                <main class="min-h-96 grid place-items-center">
                    <div class="w-full">{children()}</div>
                </main>
            </div>
        </div>
    }
}
