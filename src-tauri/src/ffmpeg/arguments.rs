use crate::converter::options;

pub fn gen_palette(option: &options::Options) -> Vec<&str> {
    let vf = format!("fps={},scale={}:-1:flags={}", &option.fps, &option.scale, &option.flags_gen);
    let vf_clone = vf.clone();
    let args = vec![
        "-y",
        "-ss", &option.start_at_second,
        "-t", &option.length_of_gif_video,
        "-i", &option.input_file,
        "-vf", &vf_clone,
        &option.palette_file,
    ];
    args
}

pub fn use_palette(option: &options::Options) -> Vec<&str> {
    let fc = format!(r#""fps={},scale={}:-1:flags={}""#, &option.fps, &option.scale, &option.flags_gen);
    let fc_clone = fc.clone();

    let args: Vec<&str> = vec![
        "-ss", &option.start_at_second,
        "-t", &option.length_of_gif_video,
        "-i", &option.input_file,
        "-i", &option.palette_file,
        "-filter_complex", &fc_clone,
        &option.output_file,
    ];

    args
}