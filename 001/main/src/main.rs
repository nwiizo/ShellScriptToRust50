use std::fs;
use std::path::Path;

fn list_directory(path: &Path) -> std::io::Result<Vec<String>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| String::from("Invalid Unicode"));

        let file_type = if entry.path().is_dir() {
            "Directory"
        } else {
            "File"
        };

        let metadata = entry.metadata()?;
        let size = metadata.len();

        let entry_info = format!("{} ({}, {}B)", file_name, file_type, size);

        entries.push(entry_info);
    }

    Ok(entries)
}

fn main() -> std::io::Result<()> {
    let current_dir = Path::new(".");
    let entries = list_directory(current_dir)?;

    println!("Directory contents:");
    for entry in entries {
        println!("{}", entry);
    }

    Ok(())
}
