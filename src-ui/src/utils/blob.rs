use leptos::{log, SignalGet};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, RtcDataChannel};

pub(crate) async fn tranfer_file(
    file: web_sys::File,
    dc: leptos::ReadSignal<Option<RtcDataChannel>>,
) -> Result<(), String> {
    let dc = match dc.get() {
        Some(dc) => dc,
        None => {
            return Err(
                "data channel not found".to_string()
            );
        }
    };
    let mut idx = 0;
    // send signal
    let initial_view =
        js_sys::Uint8Array::new_with_length(1024);
    initial_view.set_index(0, idx);
    let filename = file.name();
    log!("filename: {}", filename);
    let filename_view = js_sys::Uint8Array::new(
        &wasm_bindgen::JsValue::from_str(&filename),
    );
    initial_view.set(&filename_view, 1);
    dc.send_with_array_buffer_view(&initial_view).unwrap();
    idx += 1;

    let blob_size = file.size();
    let chunk_size = 8192.0;
    let chunk_count = (blob_size / chunk_size).ceil() as u8;
    log!("chunk count: {}", chunk_count);

    let mut slice_start = 0.0;
    // let mut idx = 0;

    while slice_start <= blob_size {
        /*
        new ArrayBuffer(buffer.byte_length + idx)
        new DataView(arrayBuffer, 0, 1024)
         */

        let view =
            js_sys::Uint8Array::new_with_length(1025);

        // let chunk = file
        //     .slice_with_f64_and_f64(0.0, chunk_size)
        //     .unwrap()
        //     .array_buffer();
        // let chunk = JsFuture::from(chunk)
        //     .await
        //     .unwrap()
        //     .dyn_into::<js_sys::ArrayBuffer>()
        //     .unwrap();
        let chunk = file
            .slice_with_f64_and_f64(
                slice_start,
                slice_start + chunk_size,
            )
            .unwrap();
        log!("chunk size: {:?}", chunk.size());
        // let chunk_view = js_sys::Uint32Array::new(&chunk);

        // let u32_array = chunk_view.to_vec();
        // log!("u32 array: {:?}", u32_array);
        // log!("index: {}", idx);
        // view.set_index(0, idx);
        // view.set(&chunk, 1);
        // log!("view.0: {:?}", view.get_index(0));
        let fr = web_sys::FileReader::new().unwrap();
        let dc_clone = dc.clone();
        let onload = wasm_bindgen::closure::Closure::wrap(
            Box::new(move |event: web_sys::Event| {
                let element = event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::FileReader>()
                    .unwrap();
                match element.ready_state() {
                    0 => {
                        log!("file reader ready state: empty")
                    }
                    1 => {
                        log!("file reader ready state: loadign")
                    }
                    2 => {
                        let data =
                            element.result().unwrap();
                        // let blob = data
                        // .dyn_into::<js_sys::ArrayBuffer>()
                        // .unwrap();
                        // log!("blob: {:?}", blob);
                        // let chunk_view =
                        //     js_sys::Uint32Array::new(&blob);

                        // let u32_array = chunk_view.to_vec();
                        // log!("u32 array: {:?}", u32_array);

                        let view =
                            js_sys::Uint8Array::new_with_length(
                                8193
                            );
                        log!(
                            "view length: {:?}",
                            view.byte_length()
                        );

                        let u8_array =
                            js_sys::Uint8Array::new(&data);

                        log!("index: {}", idx);
                        view.set_index(0, idx);
                        view.set(&u8_array, 1);
                        log!(
                            "view.0: {:?}",
                            view.get_index(0)
                        );

                        dc_clone
                            .send_with_array_buffer_view(
                                &view,
                            )
                            .unwrap();

                        let u8_array = view.to_vec();
                        log!("u8 array: {:?}", u8_array);

                        element.abort();
                    }
                    _ => unreachable!(),
                };
                // let blob: web_sys::Blob =
                //     data.into();
                // log!("blob: {:?}", blob);
                // let file_string = data
                //     .dyn_into::<web_sys::Blob>()
                //     .unwrap();
                // let file_vec: Vec<u8> =
                //     file_string
                //         .iter()
                //         .map(|x| x as u8)
                //         .collect();
                // log!(
                //     "file read: {:?}",
                //     file_string
                // );

                // let dc = match dc.get() {
                //     Some(dc) => dc,
                //     None => {
                //         log!("data channel not found");
                //         return;
                //     }
                // };

                // dc.send_with_blob(&data.into())
                //     .unwrap();
            }) as Box<dyn FnMut(_)>,
        );
        fr.set_onloadend(Some(
            onload.as_ref().unchecked_ref(),
        ));
        onload.forget();

        fr.read_as_array_buffer(&chunk).unwrap();

        // log!("view: {:?}", view);
        // log!("chunk: {:?}", chunk.byte_length());
        // let dataview =
        //     js_sys::DataView::new(&chunk, 0, 1024);
        // dataview.set_uint8(0, 1);
        // log!("data view: {:?}", dataview);
        // log!("chunk: {:?}", chunk.size());
        // log!("chunk: {:?}", chunk);
        slice_start += chunk_size;
        // log!("remaining chunk: {}", remaining_chunk);
        // fr.read_as_text(&chunk).unwrap()
        // let dc = match dc.get() {
        //     Some(dc) => dc,
        //     None => {
        //         log!("data channel not found");
        //         return Err(
        //             "data channel not found".to_string()
        //         );
        //     }
        // };

        // dc.send_with_array_buffer_view(&view).unwrap();
        // idx += 1;
        idx += 1;
    }
    Ok(())
}
