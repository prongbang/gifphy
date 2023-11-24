pub mod options;

use tauri::Window;
use crate::{command, ffmpeg, status};

pub async fn execute<'a, R: tauri::Runtime>(window: &'a Window<R>, option: &'a options::Options) {
    let ffmpeg_path = ffmpeg::get_path(&window);

    let on_converter_palette = |value: command::Value| {
        if value.status == status::IN_PROGRESS {
            #[cfg(debug_assertions)]
            {
                println!("{}", value.text);
            }
        } else if value.status == status::DONE {
            #[cfg(debug_assertions)]
            {
                println!("Done");
            }

            window.emit(command::CONVERTER_EVENT, command::Payload {
                progress: 100.0,
                status: status::DONE.to_string(),
                logs: "".to_string(),
                error: "".to_string(),
            }).unwrap();
        } else {
            #[cfg(debug_assertions)]
            {
                println!("Error: {}", value.error);
            }
            window.emit(command::CONVERTER_EVENT, command::Payload {
                progress: 0.0,
                status: status::ERROR.to_string(),
                logs: "".to_string(),
                error: value.error.clone(),
            }).unwrap();
        }
    };

    let on_generate_palette = |value: command::Value| {
        if value.status == status::IN_PROGRESS {
            #[cfg(debug_assertions)]
            {
                println!("{}", value.text);
            }
        } else if value.status == status::DONE {
            #[cfg(debug_assertions)]
            {
                println!("Done");
            }

            ffmpeg::converter_palette(
                &window,
                command::CONVERTER_STOP_EVENT,
                ffmpeg_path.as_str(),
                &option,
                on_converter_palette,
            )
        } else {
            #[cfg(debug_assertions)]
            {
                println!("Error: {}", value.error);
            }
            window.emit(command::CONVERTER_EVENT, command::Payload {
                progress: 0.0,
                status: status::ERROR.to_string(),
                logs: "".to_string(),
                error: value.error.clone(),
            }).unwrap();
        }
    };

    ffmpeg::generate_palette(
        &window,
        command::CONVERTER_STOP_EVENT,
        ffmpeg_path.as_str(),
        &option,
        on_generate_palette,
    )
}