mod archiver;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rat")]
#[command(author = "Abdelrahman ElShafay")]
#[command(version = "0.0.1")]
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
        #[arg(short = 'f', long = "name",required = true)]
        _arch_name: String,

        // paths to be stored inside the archive
        #[arg(short = 'p', long = "paths", required = true)]
        _paths: Vec<String>
    },

    // extracting files/directories from a .rat archive
    Extract {
        // file name for the .rat Archive 
        #[arg(short = 'f', required = true)]
        _arch_name: String,

        // Output Directory 
        #[arg(long ="output-dir")]
        _output_dir: String
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CLI::parse();

    match args.command {
        Commands::Create { _arch_name, _paths } => {
            match archiver::archive_file(_arch_name, &_paths) {
                Ok(_) => {},
                Err(error) => print!("{}\n", error),
            };
        } 

        Commands::Extract { _arch_name, _output_dir } => {
            archiver::extract_file(_arch_name, _output_dir)?;
        }
    }

    Ok(())

}
