use std::fs;

use clap::{ Parser, Subcommand };

use crate::run_pycn_file;

#[derive(Parser)]
#[command(name = "pycn", version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: SubCommands
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    #[command(name = "run", about = "Run code from .pycn file.")]
    Run { 
        #[arg(short, long)]
        file: Option<String>
    }
}

pub fn use_cli() {
    let cli = Cli::parse();

    match cli.cmd {
        SubCommands::Run { file } => {
            let has_main_pycn = fs::exists("./main.pycn").unwrap();
            let has_main = fs::exists("./main.py").unwrap();

            if let Some(file) = file {            
                run_pycn_file(&file);
                return
            } 
            
            if has_main_pycn {
                run_pycn_file("./main.pycn");
            } else if has_main {
                run_pycn_file("./main.py");
            } else {
                println!("No main.py or main.pycn found in this directory, if you want to choose a file, please use -f or --file.")
            }
        },
    }
}