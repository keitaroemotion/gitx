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
    validate_args_first(&args, 2);

    match &args[1][..] {
        "add" => {
                     validate_args(&args, 4);
                     gitx::add_alias(&args[2], &args[3]);
                 },
        "run" => { 
                     validate_args(&args, 4);
                     gitx::operate_with_alias();
                 },
        _     => {
                     gitx::show_branches_with_aliases();
                 }
    }
}

fn validate_args(args: &Vec<String>, len: usize) {
    if args.len() < len {
       println!("\nArgument missing. (it should be at least {}.)\n", len);
       process::exit(1);
    }
}

fn validate_args_first(args: &Vec<String>, len: usize) {
    if args.len() < len {
        show_help();
    }
}

fn show_help() {
       println!("\n\

gitx add [branch name] [alias name] ... add alias to the branch 
       ");
       process::exit(1);
}
