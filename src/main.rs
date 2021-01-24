use std::env;
use std::process;

fn main() {
    //
    // This app is for the devs who is working in a team which rules git branch name as
    // [issue-index], without allowing any detail description of the branch.
    // (Actually the best solution is just name branch description in the branch name though)
    //
    // 1. add alias (<git branch name>, <alias>)
    // 2. show branches with alias
    // 3. git operation with alias

    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "add" => {
                     validate_args(&args, 4);
                     if let result = gitx::add_alias(&args[2], &args[3]) {
                         match result {
                            Err(e) => { println!("\n{}\n", e);
                                        process::exit(1);
                                      },
                            _      => { () }
                         }
                     }
                 },
        "run" => { 
                     validate_args(&args, 4);
                     if let e = gitx::operate_with_alias() {

                     }
                 },
        _     => {
                     if let e = gitx::show_branches_with_aliases() {

                     }
                 }
    }
}

fn validate_args(args: &Vec<String>, len: usize) {
    if args.len() < len {
       println!("\nArgument missing. (it should be at least {}.)\n", len);
       process::exit(1);
    }
}
