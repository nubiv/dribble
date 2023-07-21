use leptos::{
    component, log, use_context, view, IntoView, Scope,
    SignalSet,
};

use crate::{
    app::{
        AppState, AppStateContext, Role, RoleContext,
        RtcConnectionContext,
    },
    rtc::init_connection,
};

#[component]
pub(crate) fn LandingPage(cx: Scope) -> impl IntoView {
    let set_app_state =
        use_context::<AppStateContext>(cx).unwrap().1;
    let set_role =
        use_context::<RoleContext>(cx).unwrap().1;
    let set_rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().1;

    let on_initiator = move |_| {
        match init_connection() {
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
        match init_connection() {
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
                    class="bg-blue-400 text-white rounded-lg p-2 mt-auto mb-2 hover:bg-gray-600"
                    on:click=on_initiator
                    >
                    "Initiator"
                </button>
                <button
                    class="bg-blue-400 text-white rounded-lg p-2 mb-auto hover:bg-gray-600"
                    on:click=on_responder
                    >
                    "Responder"
                </button>
            </div>
        </div>
    }
}
