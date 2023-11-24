use serde::{Serialize, Deserialize};
use crate::utils;

#[derive(Clone, Serialize, Deserialize)]
pub struct Options {
    pub input_file: String,
    pub output_file: String,
    pub output_path: String,
    pub palette_file: String,
    pub start_at_second: String,
    pub length_of_gif_video: String,
    pub fps: String,
    pub scale: String,
    pub flags_gen: String,
    pub flags_use: String,
}

impl Options {
    pub fn get_output_file_path(&self) -> String {
        let input_file_name = utils::file_util::get_file_name(&self.input_file);
        let path = format!("{}/{}.gif", self.output_path, input_file_name);
        path.replace("//", "/")
    }
}