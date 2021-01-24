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

    if args.len() < 1 {
       println!("\nargument missing.\n");
       process::exit(1);
    }

    match &args[1][..] {
        "add" => gitx::add_alias(),
        "run" => gitx::operate_with_alias(),
        _     => gitx::show_branches_with_aliases()
    }
}
