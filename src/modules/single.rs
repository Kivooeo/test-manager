use crate::{Args, FileOperation};

pub fn prepare_single_operation(
    args: &Args,
) -> Result<Vec<FileOperation>, Box<dyn std::error::Error>> {
    if args.args.is_empty() {
        return Err("Source file required".into());
    }

    let source = &args.args[0];
    let new_name = args.new_name.as_ref().ok_or("New name required (-n)")?;
    let path = args.path.as_ref().ok_or("Path required (-p)")?;

    let current_path = std::path::PathBuf::from("tests")
        .join("ui")
        .join("issues")
        .join(source);
    let destination_path = std::path::PathBuf::from("tests")
        .join("ui")
        .join(path)
        .join(new_name);

    Ok(vec![FileOperation {
        source: source.clone(),
        destination: new_name.clone(),
        current_path,
        destination_path,
    }])
}
