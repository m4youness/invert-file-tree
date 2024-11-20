use std::fs::{self, ReadDir};
use std::io::stdin;
use tempfile::NamedTempFile;

struct Tree {
    path: String,
    children: Vec<Tree>,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Tree {}

// Implement PartialOrd and Ord for Tree
impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

struct Helper {}

impl Helper {
    fn to_vec(entries: Result<ReadDir, std::io::Error>) -> Vec<String> {
        match entries {
            Ok(dir) => dir
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path().display().to_string())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn rename_file(dst: &str, src: &str) {
        match std::fs::rename(dst, src) {
            Ok(_) => println!("Renamed file {} to {}", dst, src),
            Err(_) => println!("Couldn't rename file {} to {}", dst, src),
        }
    }

    fn temp_file() -> String {
        let temp_file = NamedTempFile::new().unwrap();
        temp_file.path().display().to_string()
    }

    fn is_dir(path: &str) -> bool {
        match std::fs::metadata(path) {
            Ok(m) => m.is_dir(),
            Err(_) => false,
        }
    }
}

impl Tree {
    fn new(path: &String) -> Tree {
        Tree {
            path: path.to_string(),
            children: Vec::new(),
        }
    }

    fn add(&mut self, tree: Tree) {
        self.children.push(tree);
    }

    fn print(&self, depth: usize) {
        println!("{}{}", " ".repeat(depth * 2), self.path);

        for child in &self.children {
            child.print(depth + 1);
        }
    }
    fn to_tree(tree: &mut Tree, entry_path: &str) {
        let entries = Helper::to_vec(fs::read_dir(entry_path));
        if entries.is_empty() {
            return;
        }

        for (i, entry) in entries.iter().enumerate() {
            tree.add(Tree::new(entry));
            Self::to_tree(&mut tree.children[i], entry);
        }
    }

    fn generate_file_tree(path: &str, depth: usize) {
        if depth == 0 {
            return;
        }

        if let Err(e) = fs::create_dir(path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("Failed to create directory {}: {}", path, e);
                return;
            }
        }

        let left_child = format!("{}/left", path);
        let right_child = format!("{}/right", path);

        Self::generate_file_tree(&left_child, depth - 1);
        Self::generate_file_tree(&right_child, depth - 1);
    }
}

fn reverse(tree: &mut Tree) {
    for branch in &mut tree.children {
        if Helper::is_dir(&tree.path) {
            reverse(branch);
        }
    }
    tree.children.sort();

    let mut start = 0;
    let mut end = tree.children.len().saturating_sub(1);

    while start < end {
        let p1 = tree.children[start].path.clone();
        let p2 = tree.children[end].path.clone();
        let temp_path = Helper::temp_file();

        Helper::rename_file(&p1, &temp_path);
        Helper::rename_file(&p2, &p1);
        Helper::rename_file(&temp_path, &p2);

        start += 1;
        end = end.saturating_sub(1);
    }
}

fn main() {
    let mut input_path = String::new();

    println!("Enter a root directory.");
    match stdin().read_line(&mut input_path) {
        Ok(_) => (),
        Err(e) => println!("ERROR: {}", e),
    }

    let input_path = input_path.trim();

    // let mut depth = String::new();
    //
    // println!("Enter a depth.");
    // match stdin().read_line(&mut depth) {
    //     Ok(_) => (),
    //     Err(e) => println!("ERROR: {}", e),
    // }
    //
    // let depth: usize = match depth.trim().parse() {
    //     Ok(d) => d,
    //     Err(_) => {
    //         println!("Depth must be a number.");
    //         3
    //     }
    // };

    // Tree::generate_file_tree(input_path, depth);

    let mut tree = Tree::new(&input_path.to_string());

    Tree::to_tree(&mut tree, input_path);
    // tree.print(depth);

    reverse(&mut tree);
}
