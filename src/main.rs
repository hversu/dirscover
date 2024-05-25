use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;

// Parse the optional whitelist argument
fn parse_whitelist(whitelist_arg: Option<&String>) -> HashSet<Pattern> {
    if let Some(arg) = whitelist_arg {
        arg.split(',')
            .map(|s| Pattern::new(s.trim()).expect("Invalid pattern"))
            .collect()
    } else {
        HashSet::new()
    }
}

fn is_whitelisted(path: &Path, whitelist: &HashSet<Pattern>) -> bool {
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        return whitelist.iter().any(|pattern| pattern.matches(&file_name_str));
    }
    false
}

fn random_walk(
    path: &Path,
    terminal_paths: &mut HashSet<PathBuf>,
    visited_dirs: &mut HashSet<PathBuf>,
    max_steps: usize,
    whitelist: &HashSet<Pattern>,
) -> Option<PathBuf> {
    for _ in 0..max_steps {
        if path.is_dir() {
            let entries: Vec<PathBuf> = fs::read_dir(path)
                .expect("Unable to read directory")
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| !is_whitelisted(path, whitelist))
                .collect();

            if entries.is_empty() {
                terminal_paths.insert(path.to_path_buf());
                return None;
            } else if let Some(choice) = entries.choose(&mut thread_rng()) {
                visited_dirs.insert(choice.clone());
                return random_walk(&choice, terminal_paths, visited_dirs, max_steps, whitelist);
            }
        } else if path.is_file() {
            return Some(path.to_path_buf());
        }
    }
    None
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: {} <root_directory> <N> [whitelist]", args[0]);
        return;
    }

    let root_dir = Path::new(&args[1]);
    let n: usize = args[2].parse().expect("Integer argument for N is invalid");
    let whitelist = parse_whitelist(args.get(3));

    let max_steps = 1000;

    let mut selected_paths = Vec::new();
    let mut terminal_paths = HashSet::new();
    let mut visited_dirs = HashSet::new();

    for _ in 0..n {
        if let Some(path) = random_walk(root_dir, &mut terminal_paths, &mut visited_dirs, max_steps, &whitelist) {
            selected_paths.push(path);
        } else if terminal_paths.len() == visited_dirs.len() {
            eprintln!("All traversed directories are terminal or whitelisted.");
            break;
        }

        // Collect non-terminal directories
        let non_terminal_dirs: Vec<PathBuf> = visited_dirs.iter()
            .filter(|d| !terminal_paths.contains(*d))
            .cloned()
            .collect();

        // Re-visit directories if necessary
        for dir in non_terminal_dirs {
            if let Some(path) = random_walk(&dir, &mut terminal_paths, &mut visited_dirs, max_steps, &whitelist) {
                selected_paths.push(path);
            } else {
                terminal_paths.insert(dir);
            }

            if selected_paths.len() >= n {
                break;
            }
        }

        if selected_paths.len() >= n {
            break;
        }
    }

    // Print the selected paths
    for path in selected_paths {
        println!("{}", path.to_string_lossy());
    }
}
