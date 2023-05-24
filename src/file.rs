use dirs::config_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn create_file() {
    let mut file_path = config_dir().expect("Could not find config directory.");
    file_path.push("tcli.txt");

    if file_path.exists() {
        return;
    } else {
        let file = File::create(file_path).expect("Could not create file.");
        println!("File created: {:?}", file);
    }
}

pub fn get_tasks_from_file() -> Vec<String> {
    let file = File::open(config_dir().unwrap().join("tcli.txt")).unwrap();
    let buf = BufReader::new(file);

    buf.lines()
        .map(|x| x.expect("Could not parse line."))
        .collect()
}
