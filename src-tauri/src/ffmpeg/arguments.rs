use crate::converter::options;

pub fn gen_palette(option: &options::Options) -> Vec<String> {
    let vf = format!("fps={},scale={}:-1:flags={}", &option.fps, &option.scale, &option.flags_gen);

    let args = vec![
        "-y".to_string(),
        "-ss".to_string(), option.start_at_second.clone(),
        "-t".to_string(), option.length_of_gif_video.clone(),
        "-i".to_string(), option.input_file.clone(),
        "-vf".to_string(), vf.clone(),
        option.palette_file.clone(),
    ];

    args
}

pub fn use_palette(option: &options::Options) -> Vec<String> {
    let fc = format!("fps={},scale={}:-1:flags={}", &option.fps, &option.scale, &option.flags_use);

    let args = vec![
        "-y".to_string(), // Force overwrite
        "-ss".to_string(), option.start_at_second.clone(),
        "-t".to_string(), option.length_of_gif_video.clone(),
        "-i".to_string(), option.input_file.clone(),
        "-i".to_string(), option.palette_file.clone(),
        "-filter_complex".to_string(), fc.clone(),
        option.output_file.clone(),
    ];

    args
}