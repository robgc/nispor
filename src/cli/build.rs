use clap::Shell;
use std::env;
use std::path::Path;

include!("cli.rs");

fn main() {
    let cmd_name = "npc";
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let outdir_path = Path::new(&outdir);

    let mut n_ancestors = 0;
    let mut ancestors = outdir_path.ancestors();
    let mut output_path = None;

    while n_ancestors < 4 {
        output_path = ancestors.next();
        n_ancestors += 1;
    }

    let output_path_str = output_path.unwrap().as_os_str();

    let mut app = build_cli();

    app.gen_completions(cmd_name, Shell::Bash, output_path_str);
    app.gen_completions(cmd_name, Shell::Zsh, output_path_str);
    app.gen_completions(cmd_name, Shell::Fish, output_path_str);
}
