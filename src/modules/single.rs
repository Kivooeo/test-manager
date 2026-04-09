use crate::{Args, FileOperation};
use std::path::PathBuf;

pub fn prepare_single_operation(
    args: &Args,
) -> Result<Vec<FileOperation>, Box<dyn std::error::Error>> {
    if args.args.is_empty() {
        return Err("Source file required".into());
    }

    let source = &args.args[0];
    let new_name = args.new_name.as_ref().ok_or("New name required (-n)")?;
    let path = args.path.as_ref().ok_or("Path required (-p)")?;

    let mut current_path_buff=PathBuf::from("tests");
    current_path_buff.push("ui");
    current_path_buff.push("issues");
    current_path_buff.push(source);
    let current_path=current_path_buff.to_string_lossy().into_owned();

    let mut dest_path_buff=PathBuf::from("tests");
    dest_path_buff.push("ui");
    dest_path_buff.push(path);
    dest_path_buff.push(new_name);
    let destination_path=dest_path_buff.to_string_lossy().into_owned();

    
    Ok(vec![FileOperation {
        source: source.clone(),
        destination: new_name.clone(),
        current_path,
        destination_path,
    }])
}
