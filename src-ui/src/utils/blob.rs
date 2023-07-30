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
    log!("chunk count: {}", chunk_count);

    let mut idx = 0;

    // send signal
    let initial_view =
        js_sys::Uint8Array::new_with_length(chunk_size as u32);
    initial_view.set_index(0, idx as u8);

    let (count_view, count_view_len) = u8_arr(chunk_count);
    initial_view.set(&count_view, 1);

    let (filename_view, _) =
        u8_arr(file.name());
    initial_view
        .set(&filename_view, count_view_len + 1);
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

                        let (idx_view, idx_view_len) =
                            u8_arr(idx);
                        let aggr_view =
                            js_sys::Uint8Array::new_with_length(
                                  idx_view_len + 8192 
                            );
                        log!(
                            "view length: {:?}",
                            aggr_view.byte_length()
                        );

                        aggr_view.set(&idx_view, 0);

                        let data_view =
                            js_sys::Uint8Array::new(&data);
                        aggr_view.set(&data_view, idx_view_len );

                        dc_clone
                            .send_with_array_buffer_view(
                                &aggr_view,
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

fn u8_arr(target: impl ToString) -> ( js_sys::Uint8Array, u32 ) {
    let str = target.to_string();
    let u8_arr = str.as_bytes();
    
    let view = js_sys::Uint8Array::from(u8_arr);
    let len = view.length();

    let aggr_view = js_sys::Uint8Array::new_with_length( len + 1 );
    aggr_view.set_index(0, len as u8);
    aggr_view.set(&view, 1);
    let aggr_len = aggr_view.length();
    log!("aggr view: {:?}", aggr_view.to_vec());

    (aggr_view, aggr_len)
}