use tauri::{Manager, Window};

pub fn get_absolute_path<R: tauri::Runtime>(window: &Window<R>, path: &str) -> String {
    let handle = window.app_handle();
    let resource_path = handle.path_resolver()
        .resolve_resource(path)
        .expect("failed to resolve resource");
    let absolute_path = resource_path.as_path().display().to_string();
    absolute_path
}