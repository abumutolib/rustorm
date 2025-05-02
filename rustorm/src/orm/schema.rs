use std::fs;
use std::path::Path;

pub fn run_migrations(path: &str) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(Path::new(path))? {
        let entry = entry?;
        let content = fs::read_to_string(entry.path())?;
        println!("Running migration: {}", entry.file_name().to_string_lossy());
        println!("SQL: {}", content);
    }
    Ok(())
}