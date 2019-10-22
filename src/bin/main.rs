use file_hierarchy::Directory;

mod printing_press;

fn main() {
    
    let starting_dir_name = "./";
    let mut starting_dir = Directory::initialise(
        starting_dir_name.to_string());

    starting_dir.parse_dir();

    use crate::printing_press::press;

    println!(".");
    let base_string = String::from("");
    press::print_directory(&starting_dir, 0, &base_string);

    let size_float = starting_dir.size as f64;
    println!("Total Size of Dir: {}M", size_float * 0.000001)
}

