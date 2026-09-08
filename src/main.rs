use clap::Parser;
use piku::{assembler::Assembler, cpu::CPU, encoder::encode};

#[derive(clap::Parser)]
#[command(name = "PIKU", version = "0.1.0", about = "A 16-bit CPU architecture with a custom instruction set")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// runs a .piku file
    Run {
        /// the file to run
        #[arg(value_name = "FILE")]
        file: String,
        /// print optional debug info
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { debug, file } => {
            let src = match std::fs::read_to_string(file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to read file: {e}");
                    return;
                },
            };

            let mut assembler = Assembler::new(src);
            let items = match assembler.process() {
                Ok(i) => i,
                Err(e) => {
                    println!("An error ocurred while parsing the assembly: {e}");
                    return;
                },
            };
            if *debug {
                println!("{:?}", items);
            }

            let program = match encode(items) {
                Ok(v) => v,
                Err(e) => {
                    println!("An error occured during enocding: {e}");
                    return;
                },
            };
            if *debug {
                println!("{:?}", program);
            }

            let mut cpu = CPU::new();
            cpu.load(program);
            if let Err(e) = cpu.run(*debug) {
                println!("{e}");
                return;
            };
            if *debug {
                println!("{:?}", cpu.reg);
            }
        }
    }
}
