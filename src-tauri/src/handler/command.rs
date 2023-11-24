use tauri::Window;
use tokio::task;
use crate::{converter, ffmpeg, resources};

#[tauri::command]
pub async fn converter<R: tauri::Runtime>(
    window: Window<R>,
    input_file: String,
    output_path: String,
) {
    let mut options = converter::options::Options {
        input_file,
        output_path,
        output_file: String::from(""),
        start_at_second: String::from("0"),
        length_of_gif_video: String::from("9999999"),
        fps: String::from("10"),
        scale: String::from("320"),
        flags_gen: String::from("lanczos,palettegen"),
        flags_use: String::from("lanczos[x];[x][1:v]paletteuse"),
        palette_file: String::from(""),
    };
    options.output_file = options.get_output_file_path();
    options.palette_file = options.get_output_file_path();

    // Create temp directory
    let tmp_frames_path = resources::get_absolute_path(&window, ffmpeg::TMP_PATH);
    let tmp_frames_path_file = format!("{}/palette.png", tmp_frames_path.as_str());
    options.palette_file = tmp_frames_path_file;

    task::spawn(async move {
        converter::execute(&window, &options).await;
    });
}