use crate::{Args, FileOperation};

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

        let current_path = format!("tests/ui/issues/{}", source);
        let destination_path = format!("tests/ui/{}/{}", path, new_name);

        operations.push(FileOperation {
            source: source.clone(),
            destination: new_name.clone(),
            current_path,
            destination_path,
        });
    }

    Ok(operations)
}
