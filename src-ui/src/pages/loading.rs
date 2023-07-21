use leptos::{component, view, IntoView, Scope};

#[component]
pub(crate) fn LoadingPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row items-center h-full w-full">
            <span class="animate-spin text-2xl text-blue-400 ml-auto">
                "↻"
            </span>
            <h1 class="text-2xl text-blue-700 p-4 mr-auto">
                " Please wait... "
            </h1>
        </div>
    }
}
