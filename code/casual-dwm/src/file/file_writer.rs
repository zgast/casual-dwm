use std::io::{ Write};

pub fn write_to_file(path: &str, contents : String) {

    let mut f = std::fs::OpenOptions::new().write(true).open(path).unwrap();

    match f.write_all(contents.as_bytes()){
        Ok(_) => {println!("wrote to {}", path)}
        Err(error) => panic!("Problem writing to the file: {:?}", error)
    }
    match f.flush(){
        Ok(_) => {}
        Err(error) => panic!("Problem flushing file: {:?}", error)
    }
}