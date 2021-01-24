use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

pub fn read_alias() {
    let condfig_file = ".git/alias";

    if let Ok(lines) = read_lines(condfig_file) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    } else {
        create_file(condfig_file)
    }
}

pub fn create_file(condfig_file: &str) {
    let path    = Path::new(condfig_file);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all("".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_)    => println!("successfully wrote to {}", display),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//
// add <branch-name> <alias>
//
pub fn add_alias() {
    read_alias();
}

//
// run <action> <args>
//
pub fn operate_with_alias() {
}

//
// list <opt: -a>
//
pub fn show_branches_with_aliases() {
    println!("show_branches_with_aliases");
}