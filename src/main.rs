use std::{
    fs::{self, read_to_string},
    io::{self},
    path::{Path, PathBuf},
};

mod grep;
mod parse;

fn main() {
    // Parse args
    let cli_args = match parse::CliArgs::parse(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            return;
        }
    };

    let mut files_to_search = Vec::new();

    for input_path in &cli_args.paths {
        if collect_files(input_path, cli_args.config.recursive, &mut files_to_search).is_err() {
            continue;
        }
    }

    for path in &files_to_search {
        // Read file contents to string
        let contents = match read_to_string(path) {
            Ok(contents) => contents,
            Err(_) => continue,
        };

        // Grep call
        let matches = grep::grep(&contents, &cli_args.pattern, &cli_args.config);

        // Get match count and print
        if cli_args.config.count_matches {
            println!("{} -- Match Count: {}", path.display(), matches.len());
        }

        // Get grepped lines and print
        for (line_number, line) in matches {
            if cli_args.config.line_numbers {
                println!("{} -- Line {line_number}: {line}", path.display());
            } else {
                println!("{} -- {line}", path.display());
            }
        }
    }
}

// Helper to handle files and directories (searching for files recursively)
fn collect_files(path: &Path, recursive: bool, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let file_type = fs::symlink_metadata(path)?.file_type();

    if file_type.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }

    if !file_type.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path is not a regular file or directory",
        ));
    }

    if !recursive {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path is a directory; use -r to search directories",
        ));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child_path = entry.path();
        let child_type = entry.file_type()?;

        if child_type.is_dir() {
            collect_files(&child_path, recursive, files)?;
        } else if child_type.is_file() {
            files.push(child_path);
        }
    }

    Ok(())
}
