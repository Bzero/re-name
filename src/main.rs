mod cli;

use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use regex::bytes::RegexBuilder;
use walkdir::WalkDir;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process;

/// Main.
fn main() {
    let success = inner_main();

    let exit_code = match success {
        Ok(n) => {
            println!("Renamed {} files or folders.", n);
            0
        }
        Err(e) => {
            eprintln!("Aboring.");
            eprintln!("{}", e);
            1
        }
    };

    process::exit(exit_code);
}

/// Inner main. Performs the actual work and returns the number of renamed files and folders or an error.
fn inner_main() -> Result<u32> {
    let options = cli::Options::parse();

    let name_map = get_name_map(&options.source, &options.destination, &options)?;

    if !name_map.values().all_unique() {
        return Err(anyhow!(duplicate_msg(name_map)));
    }

    if !name_map.values().chain(name_map.keys()).all_unique() {
        return Err(anyhow!(map_to_source_msg(name_map)));
    }

    if !options.force && name_map.values().any(|b| std::fs::exists(b).unwrap_or(true)) {
        return Err(anyhow!(destination_exists_msg(name_map)));
    }

    let mut stdout = io::stdout().lock();
    let mut n_renamed: u32 = 0;

    if name_map.is_empty() {
        writeln!(stdout, "No files or folders match the source pattern.")?;
        return Ok(0);
    }

    for (source, destination) in name_map.iter() {
        if options.verbose || options.preview {
            writeln!(
                stdout,
                "rename '{}' ⇒ '{}'",
                source.to_string_lossy(),
                destination.to_string_lossy()
            )?;
        }

        if !options.preview {
            //TODO: This is prone to time-of-check, time-of-use errors.
            if options.force || !(std::fs::exists(destination).unwrap_or(true)) {
                std::fs::rename(source, destination)?;
                n_renamed += 1;
            } else {
                writeln!(
                    stdout,
                    "'{}' already exists, will not overwrite.",
                    destination.to_string_lossy()
                )?;
            }
        } else {
            n_renamed += 1;
        }
    }

    return Ok(n_renamed);
}

/// Find the conflicting mappings in name_map.
fn duplicate_msg(name_map: BTreeMap<PathBuf, PathBuf>) -> String {
    let duplicates: HashSet<PathBuf> = name_map.values().duplicates().cloned().collect();
    let mut duplicate_map: HashMap<&PathBuf, Vec<PathBuf>> = HashMap::new();
    for (s, d) in &name_map {
        if duplicates.contains(d) {
            duplicate_map.entry(d).or_default().push(s.clone());
        };
    }

    let mut msg = String::from("Multiple sources map to the same destination.\n");
    for (a, b) in duplicate_map {
        msg += &format!("The following sources all map to '{}':\n", a.to_string_lossy());
        for c in b {
            msg += &format!("'{}'\n", c.to_string_lossy());
        }
    }

    return msg;
}

/// Find the existing destination files or folders.
fn destination_exists_msg(name_map: BTreeMap<PathBuf, PathBuf>) -> String {
    let mut msg = String::from("The following destinations already exist:\n");
    for (s, d) in name_map {
        if std::fs::exists(&d).unwrap_or(true) {
            msg += &format!("'{}' ⇒ '{}'\n", s.to_string_lossy(), d.to_string_lossy());
        }
    }

    return msg;
}

/// Find the sources mapping to other sources.
fn map_to_source_msg(name_map: BTreeMap<PathBuf, PathBuf>) -> String {
    let mut conflict_map: HashMap<&PathBuf, &PathBuf> = HashMap::new();
    for (s, d) in &name_map {
        if name_map.keys().contains(d) {
            conflict_map.insert(s, d);
        };
    }

    let mut msg = String::from("Some sources map to another sources.\nThe following files or folders are conflicting:\n");
    for (a, b) in conflict_map {
        msg += &format!("'{}' ⇒ '{}'\n", a.to_string_lossy(), b.to_string_lossy());
    }

    return msg;
}

/// Get the map of sources to destinations.
fn get_name_map(
    source_pattern: &String,
    destination_pattern: &String,
    options: &cli::Options,
) -> Result<BTreeMap<PathBuf, PathBuf>> {
    let source_regex = build_regex(source_pattern)?;

    let mut walker = WalkDir::new(Path::new("./"));
    if !options.match_subdirs {
        walker = walker.max_depth(1);
    }

    let mut name_map: BTreeMap<PathBuf, PathBuf> = BTreeMap::new();

    for possible_entry in walker {
        let entry = match possible_entry {
            Ok(e) => e,
            Err(err) => {
                let path = err.path().unwrap_or(Path::new(""));
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::PermissionDenied => {
                            eprintln!("Missing permission to access '{}'.", path.display())
                        }
                        _ => {
                            eprintln!("Unable to access '{}'.", path.display())
                        }
                    }
                }
                continue;
            }
        };

        //skip root
        if entry.depth() == 0 {
            continue;
        }

        let current_path = strip_current_dir(entry.path());

        if source_regex.is_match(current_path.as_os_str().as_bytes()) {
            let source = current_path;
            let replaced = source_regex.replace(source.as_os_str().as_bytes(), destination_pattern.as_bytes());
            let destination = Path::new(OsStr::from_bytes(&replaced));

            if source != destination {
                name_map.insert(source.to_path_buf(), destination.to_path_buf());
            }
        }
    }

    return Ok(name_map);
}

/// Build Regex matching a full search string according to pattern and options.
fn build_regex(pattern: &String) -> Result<regex::bytes::Regex, regex::Error> {
    let full_pattern = String::from("^") + pattern + "$";
    return RegexBuilder::new(&full_pattern).build();
}

/// Remove the `./` prefix from a path.
pub fn strip_current_dir(path: &Path) -> &Path {
    return path.strip_prefix(".").unwrap_or(path);
}
