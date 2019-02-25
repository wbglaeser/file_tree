use std::fs;
use file_hierarchy::Directory;

fn main() {

    let starting_dir_name = "./";
    let mut starting_dir = Directory::initialise(
        starting_dir_name.to_string());

    starting_dir.parse_dir();
    //starting_dir.sum_filesizes();
}

