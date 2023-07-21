use leptos::{
    component, create_signal, use_context, view, IntoView,
    Scope, SignalGet,
};

use crate::{
    app::{Role, RoleContext},
    components::{InitiatorConfig, ResponderConfig},
};

#[derive(Clone, Default)]
pub(crate) enum MediaOption {
    #[default]
    FileTransfer,
    WithVideo,
    WithAudio,
}

#[component]
pub(crate) fn ConfigPage(cx: Scope) -> impl IntoView {
    let role = use_context::<RoleContext>(cx).unwrap().0;
    let (media_option, set_media_option) =
        create_signal(cx, MediaOption::default());

    view! { cx,
        <h1 class="text-3xl text-blue-400 p-4 mt-[20%]">
            {"configure your connection: "}
        </h1>
        {
            move || match role.get() {
                Role::Initiator => {
                    view! { cx,
                        <InitiatorConfig
                            media_option=media_option
                            set_media_option=set_media_option
                        />
                    }
                }
                Role::Responder => {
                    view! { cx,
                        <ResponderConfig
                            media_option=media_option
                            set_media_option=set_media_option
                        />
                    }
                }
            }
        }
    }
}
