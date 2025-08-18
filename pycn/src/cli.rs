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
        file: Option<String>
    }
}

pub fn use_cli() {
    let cli = Cli::parse();

    match cli.cmd {
        SubCommands::Run { file } => {
            if let Some(file) = file {   
                run_pycn_file(&file);
                return
            } else {
                eprintln!("No file provided. Please specify a .pycn or .py file to run.");
            }
        },
    }
}