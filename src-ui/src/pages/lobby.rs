use core::slice;

use leptos::{
    component, create_local_resource, create_node_ref,
    html::Input, log, use_context, view, IntoView, Scope,
    SignalGet, SignalSet,
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
        tauri_ipc::{
            invoke_open_file_folder, invoke_send_file,
            listen_on_file_transfer_event,
        },
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
            // let fr = web_sys::FileReader::new().unwrap();
            // let onload =
            //     wasm_bindgen::closure::Closure::wrap(
            //         Box::new(
            //             move |event: web_sys::Event| {
            //                 let element = event
            //                     .target()
            //                     .unwrap()
            //                     .dyn_into::<web_sys::FileReader>()
            //                     .unwrap();
            //                 let data =
            //                     element.result().unwrap();
            //                 let blob = data
            //                     .dyn_into::<web_sys::Blob>()
            //                     .unwrap();
            //                 log!("blob: {:?}", blob);
            //                 // let blob: web_sys::Blob =
            //                 //     data.into();
            //                 // log!("blob: {:?}", blob);
            //                 // let file_string = data
            //                 //     .dyn_into::<web_sys::Blob>()
            //                 //     .unwrap();
            //                 // let file_vec: Vec<u8> =
            //                 //     file_string
            //                 //         .iter()
            //                 //         .map(|x| x as u8)
            //                 //         .collect();
            //                 // log!(
            //                 //     "file read: {:?}",
            //                 //     file_string
            //                 // );

            //                 let dc = match dc.get() {
            //                     Some(dc) => dc,
            //                     None => {
            //                         log!("data channel not found");
            //                         return;
            //                     }
            //                 };

            //                 // dc.send_with_blob(&data.into())
            //                 //     .unwrap();
            //             },
            //         )
            //             as Box<dyn FnMut(_)>,
            //     );
            // fr.set_onloadend(Some(
            //     onload.as_ref().unchecked_ref(),
            // ));
            // onload.forget();

            // let blob_size = file.size();
            // let chunk_size = 1024.0;
            // let chunk_count =
            //     (blob_size / chunk_size).ceil() as u32;
            // log!("chunk count: {}", chunk_count);
            // let mut remaining_chunk = blob_size;
            // let mut slice_start = 0.0;

            // while remaining_chunk > 0.0 {
            //     let chunk = &file
            //         .slice_with_f64_and_f64(
            //             slice_start,
            //             chunk_size,
            //         )
            //         .unwrap();
            //     log!("chunk: {:?}", chunk.size());
            //     slice_start += 1024.0;
            //     remaining_chunk -= 1024.0;
            //     log!(
            //         "remaining chunk: {}",
            //         remaining_chunk
            //     );
            //     // fr.read_as_array_buffer(&chunk).unwrap();
            // }
            // TODO: slice file using blob slice
            // let chunks = file
            //     .slice_with_i32_and_i32(0, 1024)
            //     .unwrap();
            // log!("chunks: {:?}", chunks.size());

            // let value = file_input_el.value();
            // log!("file input value: {}", value);
            // let path = extract_filename(&value);
            // log!("file path: {}", path);

            // fr.read_as_binary_string(&file).unwrap();
            // fr.read_as_array_buffer(&chunks).unwrap();

            // if let Err(e) =
            //     invoke_send_file(path.to_string()).await
            // {
            //     log!("failed to send file: {}", e);
            // };
        })
    };

    fn extract_filename(path: &str) -> &str {
        if let Some(s) = path.strip_prefix("C:\\fakepath\\")
        {
            // modern browser
            return s;
        }
        if let Some(x) = path.rfind('/') {
            // Unix-based path
            return &path[(x + 1)..];
        }
        if let Some(x) = path.rfind('\\') {
            // Windows-based path
            return &path[(x + 1)..];
        }
        path
    }

    create_local_resource(
        cx,
        move || (),
        |_| listen_on_file_transfer_event(),
    );

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
