mod archiver;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rat")]
#[command(author = "Abdelrahman ElShafay")]
#[command(version = "0.1")]
#[command(about = "A custom .rat file archiver", long_about = "A tar inspired file archiver & compressor built in the rust programming language.")]

struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Creatuing a .rat archive
    Create {
        // Archive file name
        #[arg(short = 'f', required = true)]
        arch_name: String,

        // paths to be stored inside the archive
        #[arg(required = true)]
        paths: Vec<String>
    },

    // extracting files/directories from a .rat archive
    Extract {
        // file name for the .rat Archive 
        #[arg(short = 'f', required = true)]
        arch_name: String,

        // Output Directory 
        #[arg(short='C', long ="Dir")]
        output_dir: String
    }

}

fn main() {
    // usage: rat <CLI_Flags> <archive_name> <file or directory paths>

    

}
