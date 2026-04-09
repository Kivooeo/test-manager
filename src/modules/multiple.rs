use crate::{Args, FileOperation};
use std::path::PathBuf;


pub fn prepare_multi_operations(
    args: &Args,
) -> Result<Vec<FileOperation>, Box<dyn std::error::Error>> {
    if args.args.len() % 3 != 0 {
        return Err("Multi mode requires triplets of: source path newname".into());
    }

    let mut operations = Vec::new();

    for chunk in args.args.chunks(3) {
        let source = &chunk[0];
        let path = &chunk[1];
        let new_name = &chunk[2];
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
        operations.push(FileOperation {
            source: source.clone(),
            destination: new_name.clone(),
            current_path,
            destination_path,
        });
    }

    Ok(operations)
}
