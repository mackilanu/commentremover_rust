use std::fs;
use std::process::exit;

pub fn read_file(filename: &str) -> String {
   match fs::read_to_string(filename) {
         Ok(contents) => contents,
         Err(_) => {
            println!("The input file was not found.");
             exit(1);
         }
     }
}

pub fn remove_comments(contents: String) -> String {
    let mut contents_new = String::new(); 

    for line in contents.split('\n') {
        if line.contains("//") {
            let to_index = line.find("//").unwrap_or(0);
        
            if to_index > 0 {
                if !line.trim().starts_with("//") {
                    contents_new.push_str(&line[..to_index]);
                    contents_new.push('\n');
                }
            }
        } else {
            contents_new.push_str(line);
            contents_new.push('\n');
        }
    }
    
    contents_new
}