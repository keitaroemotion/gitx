use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

const CONFIG_FILE:&'static str = ".git/alias";

pub struct Alias {
    key:   String,
    value: String
}

impl Alias {
    fn new(line: &str) -> Alias {
        let mut lsp: Vec<&str> = line.split(" ").collect();
        Alias { key: lsp[0].to_string(), value: lsp[1].to_string() }
    }
}

pub fn read_alias() -> Vec<Alias> {
    let mut aliases: Vec<Alias> = Vec::new();
    if let Ok(lines) = read_lines(CONFIG_FILE) {
        for line in lines {
            if let Ok(x) = line {
                aliases.push(Alias::new(&x[..]));
            }
        }
    } else {
        create_config_file()
    }
    aliases
}

pub fn create_config_file() {
    let path    = Path::new(CONFIG_FILE);
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
pub fn add_alias(branch_name: &str, alias_name: &str) {
    let alias = read_alias(); // rather than alias struct, maybe it should be Hash
    for x in alias {
        if x.key == branch_name {
            println!("it is already in the alias file");
            process::exit(1);
        }
    }

    let mut file = OpenOptions::new().append(true)
                                     .open  (CONFIG_FILE)
                                     .expect("cannot open file");
   
    let x = format!("{} {}\n", branch_name, alias_name);
    file.write_all(x.as_bytes()).expect("write failed");
    println!("added | branch name: {}, alias: {}", branch_name, alias_name);
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
