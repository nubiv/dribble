use leptos::{log, SignalGet};
use wasm_bindgen::JsCast;
use web_sys::RtcDataChannel;

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

    let blob_size = file.size();
    let chunk_size = 8192.0;
    let chunk_count = (blob_size / chunk_size).ceil();
    let chunk_count =
        if chunk_count == 0.0 { 1.0 } else { chunk_count };

    // let multiplier = (chunk_count / 256.0).floor();
    // let modulo = chunk_count / 256.0;
    log!("chunk count: {}", chunk_count);
    let mut idx = 0;

    // send signal
    let initial_view =
        js_sys::Uint8Array::new_with_length(1024);
    initial_view.set_index(0, idx);

    // let chunk_count_view =
    //     js_sys::Uint8Array::new_with_length(3);
    // chunk_count_view.set_index(0, 0);
    // chunk_count_view.set_index(1, multiplier as u8);
    // chunk_count_view.set_index(2, modulo as u8);
    // initial_view.set(&chunk_count_view, 1);
    initial_view.set_index(1, chunk_count as u8);

    let filename = file.name();
    let u8_array = filename.as_bytes();
    let filename_view = js_sys::Uint8Array::from(u8_array);
    let filename_u8_length = filename_view.length();
    initial_view.set_index(2, filename_u8_length as u8);
    initial_view.set(&filename_view, 3);
    dc.send_with_array_buffer_view(&initial_view).unwrap();
    idx += 1;

    // send file chunks
    let mut slice_start = 0.0;
    while slice_start <= blob_size {
        let chunk = file
            .slice_with_f64_and_f64(
                slice_start,
                slice_start + chunk_size,
            )
            .unwrap();
        log!("chunk size: {:?}", chunk.size());

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

                        element.abort();
                    }
                    _ => unreachable!(),
                };
            }) as Box<dyn FnMut(_)>,
        );
        fr.set_onloadend(Some(
            onload.as_ref().unchecked_ref(),
        ));
        onload.forget();

        fr.read_as_array_buffer(&chunk).unwrap();

        slice_start += chunk_size;
        idx += 1;
    }
    Ok(())
}
