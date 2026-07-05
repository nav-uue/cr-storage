use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;


const EXAMPLES: &str = "Examples:\n  \
                        sudo crStorage create --name[-n] file_name  --size[-s] 32[MB/GB] --path[-p] /your/path\n  \
                        sudo crStorage delete --name[-n] file_name";

#[derive(Parser, Debug)]
#[command(
    name = "crStorage",
    author = "nav-uue", 
    version = "1.0.0", 
    about = "Simple encription tool", 
    long_about,
    after_help(EXAMPLES)
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    #[command(about = "Create new image file and mount him as loop device")]
    Create(CreateArgs),

    #[command(about = "Unmount and delete image file")]
    Delete(DeleteArgs),
    // Mount(MountArgs),
    // Umount(UmountArgs),
    // Info(InfoArgs)
}

#[derive(Args, Debug)]
pub struct CreateArgs {

    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub size: String,

    #[arg(short, long)]
    pub path: String

}

#[derive(Args, Debug)]
pub struct DeleteArgs {

    #[arg(short, long)]
    pub name: String

}

