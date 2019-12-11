use std::env;
use std::path::Path;
use std::fs::{self, DirEntry};

struct Entry {
    name: String,
    children: Vec<Entry> 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", render_tree(&tree(Path::new(&args[1]))).join("\n"));
}

fn children(dir: &Path) -> Vec<Entry> {
    fs::read_dir(dir)
        .expect("unable to read dir")
        .into_iter()
        .map(|e| e.expect("unable to get entry"))
        .filter(|e| is_not_hidden(e))
        .map(|e| e.path())
        .map(|e| tree(&e))
        .collect()
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| !s.starts_with("."))
         .unwrap_or(false)
}

fn tree(path: &Path) -> Entry {
    Entry{
        name: path.file_name()
            .and_then(|name| name.to_str())
            .map_or(String::from("."),|str| String::from(str)),
        children: if path.is_dir() {
            children(path)
        } else {
            Vec::new()
        }
    }
}

fn render_tree(tree: &Entry) -> Vec<String> {
    let mut names = vec![tree.name.to_owned()];
    let children = &tree.children;
    let children: Vec<_> = children
        .iter()
        .enumerate()
        .map(|(i, child)| decorate(children.len() - 1 == i, render_tree(child)))
        .flatten()
        .collect();
    
    names.extend(children);

    names
}

fn decorate(is_last: bool, children: Vec<String>) -> Vec<String> {
    const I_BRANCH: &str = "│   ";
    const T_BRANCH: &str = "├── "; 
    const L_BRANCH: &str = "└── ";
    const   SPACER: &str = "    ";

    let prefix_first = if is_last { L_BRANCH } else { T_BRANCH };

    let prefix_rest = if is_last { SPACER } else { I_BRANCH };

    let mut first = vec![format!("{}{}", prefix_first, children[0])];

    first.extend(children[1..].iter().map(|child| format!("{}{}", prefix_rest, child)).collect::<Vec<_>>());

    first
}
