use clap::Parser;

#[derive(clap::Subcommand)]
pub enum Command {
    #[command(about = "Install a package")] Install(InstallArgs),
}

#[derive(clap::Args)]
pub struct InstallArgs {
    #[clap(short, long, default_value = "default")]
    pub name: String,
}

#[derive(clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Install(install_args) => {
            let name = format!("Hello {}", install_args.name);
            println!("{}", name)
        }
    }
}
