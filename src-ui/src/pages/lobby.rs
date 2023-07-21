use leptos::{
    component, use_context, view, IntoView, Scope,
    SignalGet, SignalSet,
};
use wasm_bindgen::JsCast;
use web_sys::MediaStreamTrack;

use crate::app::{
    AppState, AppStateContext, MediaStreamContext,
    RtcConnectionContext,
};

#[component]
pub(crate) fn LobbyPage(cx: Scope) -> impl IntoView {
    let set_app_state =
        use_context::<AppStateContext>(cx).unwrap().1;
    let rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().0;
    let set_rtc_pc =
        use_context::<RtcConnectionContext>(cx).unwrap().1;
    let media_stream =
        use_context::<MediaStreamContext>(cx).unwrap().0;
    let set_media_stream =
        use_context::<MediaStreamContext>(cx).unwrap().1;

    let close_track = move || {
        if let Some(stream) = media_stream.get() {
            let tracks = stream.get_tracks();
            tracks.iter().for_each(|track| {
                track
                    .dyn_into::<MediaStreamTrack>()
                    .unwrap()
                    .stop();
            });
        };
        set_media_stream.set(None);
    };

    let on_quit_session = move |_| {
        if let Some(pc) = rtc_pc.get() {
            pc.close();
            close_track();
        };
        set_rtc_pc.set(None);
        set_app_state.set(AppState::Stable);
    };

    view! { cx,
        <div class="flex flex-col items-center h-full w-full">
            <button
                class="bg-blue-400 text-white rounded-lg p-2 mt-8 hover:bg-gray-600"
                on:click=on_quit_session
                >
                "Quit Session"
            </button>
        </div>
    }
}
