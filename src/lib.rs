use std::fs;

#[derive(Debug)]
pub struct Directory {
    pub dir_name: String,
    pub size: u64,
    pub files: Vec<String>,
    pub sub_directories: Vec<Directory>,
    pub parent_directory: String,
}

impl Directory {
    pub fn initialise(dn: String) -> Directory {
        Directory {
            dir_name: dn,
            size: 0,
            files:Vec::new(),
            sub_directories: Vec::new(),
            parent_directory: " ".to_string(),
        }
    }
}

impl Directory {
    pub fn parse_dir(&mut self){

        for p in fs::read_dir(&self.dir_name).unwrap() {

            let path = p.unwrap();
            let metadata = fs::metadata(&path.path()).unwrap();

            // Add raw bytes
            let path_string = String::from(path.file_name().to_str().unwrap());
            let path_name =
                format!("{}/{}", &self.dir_name, path_string);
            let this_v = fs::metadata(path_name.clone()).unwrap().len();
            self.size += this_v.clone();

            if metadata.is_dir()  {

                match fs::read_link(path_name.clone()) {
                    Ok(_f) => {}
                    Err(_e) => {
                        // Parse Sub Directory
                        let mut new_dir = Directory::initialise(path_name.clone());
                        new_dir.parse_dir();

                        // Update Dir Values
                        self.size += new_dir.size;
                        self.sub_directories.push(new_dir);
                    }
                }
            }
            else if metadata.is_file() {
                self.files.push(path_name.clone());
            }
        }
        print!("{}: {}\n", self.dir_name, self.size);
    }
}

