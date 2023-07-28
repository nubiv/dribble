use core::slice;

use leptos::{
    component, create_node_ref, html::Input, log,
    use_context, view, IntoView, Scope, SignalGet,
    SignalSet,
};
use wasm_bindgen::JsCast;
use web_sys::MediaStreamTrack;

use crate::{
    app::{
        AppState, AppStateContext, DataChannelContext,
        MediaStreamContext, RtcConnectionContext,
    },
    utils::{
        blob::tranfer_file,
        tauri_ipc::invoke_open_file_folder,
    },
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
    let file_input_ref = create_node_ref::<Input>(cx);
    let dc =
        use_context::<DataChannelContext>(cx).unwrap().0;

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

    let on_open_folder = move |_| {
        leptos::spawn_local(async move {
            if let Err(e) = invoke_open_file_folder().await
            {
                log!("failed to open file folder: {}", e);
            };
        })
    };

    let on_send_file = move |_| {
        leptos::spawn_local(async move {
            log!("send file");
            let file_input_el = file_input_ref
                .get()
                .expect("cant find file input element");
            let files = file_input_el.files().unwrap();
            let file =
                files.get(0).expect("no file selected");

            tranfer_file(file, dc).await.unwrap();
        })
    };

    // fn extract_filename(path: &str) -> &str {
    //     if let Some(s) = path.strip_prefix("C:\\fakepath\\")
    //     {
    //         // modern browser
    //         return s;
    //     }
    //     if let Some(x) = path.rfind('/') {
    //         // Unix-based path
    //         return &path[(x + 1)..];
    //     }
    //     if let Some(x) = path.rfind('\\') {
    //         // Windows-based path
    //         return &path[(x + 1)..];
    //     }
    //     path
    // }

    view! { cx,
        <div class="flex flex-col items-center mb-auto mt-4">
            <div
                class="flex flex-col mt-auto mx-auto w-full h-auto"
            >
                <div class="mx-auto my-1 p-2">
                    <input
                        node_ref=file_input_ref
                        type="file"
                        class="hover:cursor-pointer border-slate-500 text-center w-60 border-3 p-1 mr-2"
                    ></input>
                    <button
                        class="bg-blue-400 text-white rounded-lg py-1 px-2 hover:bg-gray-600"
                        on:click=on_send_file
                        >
                        "Send"
                    </button>
                </div>
                <div class="mx-auto my-1 p-2">
                    <input type="file" class=""></input>
                </div>
            </div>
            <button
                class="bg-blue-400 text-white mb-auto rounded-lg py-2 px-4 mt-8 hover:bg-gray-600"
                on:click=on_open_folder
                >
                "Open File Folder"
            </button>
            <button
                class="bg-blue-400 text-white mb-auto rounded-lg py-2 px-4 mt-8 hover:bg-gray-600"
                on:click=on_quit_session
                >
                "Quit Session"
            </button>
        </div>
    }
}
