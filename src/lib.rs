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
            let path_string = &path.file_name();
            let metadata = fs::metadata(&path.path()).unwrap();

            if metadata.is_dir() {

                // Parse Sub Directory
                let new_dir_name =
                    format!("{}/{}", &self.dir_name, String::from(path_string.to_str().unwrap()));
                let mut new_dir = Directory::initialise(new_dir_name.clone());
                let this_v = fs::metadata(new_dir_name.clone()).unwrap().len();
                self.size += this_v.clone();


                new_dir.parse_dir();

                // Update Dir Values
                self.size += new_dir.size;
                self.sub_directories.push(new_dir);
            }

            else if metadata.is_file() {

                // Update Files
                let filename_string =
                    String::from(path_string.to_str().unwrap()).clone();
                self.files.push(filename_string.clone());
                let filename_full =
                    format!("{}/{}", &self.dir_name, filename_string);
                self.size += fs::metadata(filename_full).unwrap().len();
            }
        }
        print!("{}: {}\n", self.dir_name, self.size);
    }
}

impl Directory {
    pub fn sum_filesizes(&self) {
        let mut total = 0;

        for v in &self.files {
            let filename_full = format!("{}/{}", &self.dir_name, v);
            let metadata = fs::metadata(filename_full).unwrap();
            total += metadata.len();
        }

        for d in &self.sub_directories {
            total += d.size;
        }
        print!("{}: {}\n", &self.dir_name, total);
    }
}


//impl Clone for Directory {
//    fn clone(&self) -> Directory { *self }
//}
