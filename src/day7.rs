use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use ego_tree::{NodeId, NodeMut, NodeRef, Tree};

#[derive(Debug, Clone)]
struct FileP {
    name: String,
    size: i64,
}

#[derive(Debug)]
enum Subfolfile {
    File(FileP),
    Folder(String)
}

#[derive(Debug)]
struct DirectoryData {
    name: String,
    total_size: Option<i64>,
    sub_files: Vec<FileP>,
}

#[derive(Debug)]
enum Command {
    CdSub(String),
    CdParent,
    CdRoot,
    Ls(Vec<Subfolfile>)
}
fn parse_command(cmd: &str) -> Command {
    let mut it_lines = cmd.trim().split('\n');
    let cmd_line: Vec<&str> = it_lines.next().unwrap().split(' ').collect();
    if cmd_line[0] == "cd" {
        match cmd_line[1] {
            ".." => Command::CdParent,
            "/" => Command::CdRoot,
            s => Command::CdSub(s.to_string())
        }

    }
    else {
        Command::Ls(it_lines.map(|f_str| {
            let file_line: Vec<&str> = f_str.split(' ').collect();
            if let Ok(size) = i64::from_str(file_line[0])  {
                Subfolfile::File(FileP { name: file_line[1].to_string(), size  })
            }
            else {
                Subfolfile::Folder(file_line[1].to_string())
            }
        }).collect())
    }
}

fn apply_command(tree: &mut Tree<DirectoryData>, curr_pos: &mut NodeId, cmd: Command){
    match cmd {
        Command::CdSub(s) => {*curr_pos = tree.get(*curr_pos).unwrap().children().find(|child| child.value().name == s).unwrap().id()}
        Command::CdParent => {*curr_pos = tree.get(*curr_pos).unwrap().parent().unwrap().id()}
        Command::CdRoot => {*curr_pos = tree.root().id()}
        Command::Ls(l) => {l.iter().for_each(|sub| {
            match sub {
                Subfolfile::File(file_p) => {tree.get_mut(*curr_pos).unwrap().value().sub_files.push(file_p.clone());}
                Subfolfile::Folder(s) => {tree.get_mut(*curr_pos).unwrap().append(DirectoryData
                { name: s.to_string(), total_size: None, sub_files: vec![] });}
            }
        })}
    }
}

fn size_node(tree: &Tree<DirectoryData>, pos: &NodeId, sizes: &mut HashMap<NodeId,i64>) -> i64 {
    let node = tree.get(*pos).unwrap();
    if let Some(u) = sizes.get(pos) {
        *u
    }
    else{
        let mut result: i64 = node.value().sub_files.iter().map(|file| file.size).sum::<i64>();
        node.children().for_each(|child| result += size_node(tree,& child.id(), sizes));
        sizes.insert(*pos,result);
        result
    }
}
pub fn day7() {
    let mut file = File::open("./inputs/input_day7.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut directory_tree = Tree::new(DirectoryData { name: String::from("/"), total_size: None, sub_files: vec![] });
    let mut curr_pos = directory_tree.root().id();
    data.split('$')
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(parse_command)
        .for_each(|v| apply_command(&mut directory_tree,&mut curr_pos,v));
    let mut table: HashMap<NodeId,i64> = HashMap::new();
    let sum = directory_tree.nodes()
        .map(|node| size_node(&directory_tree,&node.id(),&mut table))
        .filter(|&n| n <= 100_000)
        .sum::<i64>();
    let needed_space = size_node(&directory_tree,&directory_tree.root().id(),&mut table) - 40_000_000;
    let sum2 = table.values()
        .filter(|&n| *n >= needed_space)
        .min()
        .unwrap();
    println!("Solution 1 : {:?}",sum);
    println!("Solution 2 : {:?}",sum2);
}