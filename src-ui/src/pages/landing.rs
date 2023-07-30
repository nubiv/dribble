use leptos::{
    component, create_node_ref, html::Input, log,
    use_context, view, IntoView, Scope, SignalSet,
};

use crate::{
    app::{
        AppState, AppStateContext, Role, RoleContext,
        RtcConnectionContext,
    },
    utils::rtc::init_connection,
};

#[component]
pub(crate) fn LandingPage(cx: Scope) -> impl IntoView {
    let set_app_state =
        use_context::<AppStateContext>(cx).unwrap().1;
    let set_role =
        use_context::<RoleContext>(cx).unwrap().1;
    let set_rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().1;
    let stun_server_ref = create_node_ref::<Input>(cx);

    let get_stun_server = move || {
        let stun_server_el = stun_server_ref.get().unwrap();
        let value = stun_server_el.value();

        if value.is_empty() {
            "stun:stun.l.google.com:19302".to_string()
        } else {
            value
        }
    };

    let on_initiator = move |_| {
        let stun_server = get_stun_server();
        match init_connection(&stun_server) {
            Ok(pc) => {
                set_rtc_pc.set(Some(pc));

                set_role.set(Role::Initiator);
                set_app_state.set(AppState::Connecting);
            }
            Err(e) => {
                log!("error: {:?}", e);
            }
        };
    };

    let on_responder = move |_| {
        let stun_server = get_stun_server();
        match init_connection(&stun_server) {
            Ok(pc) => {
                set_rtc_pc.set(Some(pc));

                set_role.set(Role::Responder);
                set_app_state.set(AppState::Connecting);
            }
            Err(e) => {
                log!("error: {:?}", e);
            }
        };
    };

    view! { cx,
        <div class="flex flex-col items-center h-full w-full">
            <h1 class="text-3xl text-blue-400 p-4 mt-[20%]">
                {"Dribble 🌊"}
            </h1>
            <div class="flex flex-col h-[30%]">
                <button
                    class="bg-blue-400 text-white rounded-lg py-2 px-4 mt-auto mb-2 hover:bg-gray-600"
                    on:click=on_initiator
                    >
                    "Initiator"
                </button>
                <button
                    class="bg-blue-400 text-white rounded-lg py-2 px-4 mb-auto hover:bg-gray-600"
                    on:click=on_responder
                    >
                    "Responder"
                </button>
                <label class="text-blue-400">
                    "Stun Server:"
                </label>
                <input
                    node_ref=stun_server_ref
                    class="border-blue-400 border-2 rounded-lg py-2 px-4 mb-auto"
                    placeholder="Default as Google Stun Server"
                    >
                </input>
            </div>
        </div>
    }
}
