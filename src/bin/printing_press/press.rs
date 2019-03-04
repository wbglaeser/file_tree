extern crate regex;

use file_hierarchy::Directory;
use regex::Regex;

pub fn print_directory (dir: &file_hierarchy::Directory, depth: usize, base_string: &String){

    // Define Loop variables
    let index_last_sub_dir = dir.sub_directories.len();
    let mut count_var = 1;
    let offset = "    ".repeat(depth);

    // Define dir name regex
    let re = Regex::new(r"[\w\.\-_]+$").unwrap();

    // Number of sub dirs
    let len_sds = 0;

    // Loop through sub directories
    for sd in &dir.sub_directories {

        // Extract dir name
        let dir_name_end_captures = re.captures(&sd.dir_name).unwrap();
        let name = dir_name_end_captures.get(0).map_or("", |m| m.as_str());

        // Print dir and contained sub dirs
        let size_float = sd.size as f64;

        let new_base = if count_var == index_last_sub_dir {
            println!("{}└── {} -> {}M", base_string, String::from(name), size_float * 0.000001);
            format!("{}{}", base_string, String::from("    "))
        }
        else {
            count_var += 1;
            println!("{}├── {} -> {}M", base_string, String::from(name), size_float * 0.000001);
            format!("{}{}", base_string, String::from("│   "))
        };

        print_directory(&sd, depth + 1, &new_base);

    }
}