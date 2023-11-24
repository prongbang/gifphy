pub mod arguments;

use tauri::Window;
use crate::{command, resources};
use crate::converter::options::Options;

pub const TMP_PATH: &str = "resources/tmp";
const WINDOWS_BIN_PATH: &str = "resources/windows/bin/ffmpeg.exe";
const MACOS_BIN_PATH: &str = "resources/macos/bin/ffmpeg";
const LINUX_BIN_PATH: &str = "resources/linux/bin/ffmpeg";

pub fn get_path<'a, R: tauri::Runtime>(window: &Window<R>) -> String {
    let execute_path: &str = match std::env::consts::OS {
        WINDOWS => WINDOWS_BIN_PATH,
        MACOS => MACOS_BIN_PATH,
        LINUX => LINUX_BIN_PATH,
        _ => "",
    };
    let path_execute = resources::get_absolute_path(&window, execute_path);
    path_execute
}

pub fn generate_palette<F: Fn(command::Value), R: tauri::Runtime>(window: &Window<R>, stop_event: &str, ffmpeg_path: &str, option: &Options, on_value: F) {
    let args = arguments::gen_palette(&option);

    command::command(&window, stop_event, ffmpeg_path, args, on_value)
}

pub fn converter_palette<F: Fn(command::Value), R: tauri::Runtime>(window: &Window<R>, stop_event: &str, ffmpeg_path: &str, option: &Options, on_value: F) {
    let args = arguments::use_palette(&option);

    command::command(&window, stop_event, ffmpeg_path, args, on_value)
}