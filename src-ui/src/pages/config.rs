use leptos::{
    component, create_signal, use_context, view, IntoView,
    Scope, SignalGet,
};

use crate::{
    app::{Role, RoleContext},
    components::{InitiatorConfig, ResponderConfig},
};

#[component]
pub(crate) fn ConfigPage(cx: Scope) -> impl IntoView {
    let role = use_context::<RoleContext>(cx).unwrap().0;

    view! { cx,
        <h1 class="text-3xl text-blue-400 p-4 mt-[20%]">
            {"configure your connection: "}
        </h1>
        {
            move || match role.get() {
                Role::Initiator => {
                    view! { cx,
                        <InitiatorConfig />
                    }
                }
                Role::Responder => {
                    view! { cx,
                        <ResponderConfig />
                    }
                }
            }
        }
    }
}
