use std::path::Path;
use clap::{Parser, ArgAction};
use parser_code::parse_file;
use log::{info, error};
use env_logger::Env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to parse
    #[arg(short, long)]
    file: String,

    /// Verbose output
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

fn main() {
    // Initialize the logger
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    // Parse command line arguments
    let args = Args::parse();
    
    // Set log level based on verbosity
    match args.verbose {
        0 => log::set_max_level(log::LevelFilter::Info),
        1 => log::set_max_level(log::LevelFilter::Debug),
        _ => log::set_max_level(log::LevelFilter::Trace),
    }

    info!("Testing parser with file: {}", args.file);
    
    // Parse the file
    match parse_file(Path::new(&args.file)) {
        Ok(classes) => {
            println!("Successfully parsed file with {} classes", classes.len());
            
            // Print class information
            for (i, class) in classes.iter().enumerate() {
                println!("Class {}: {} (parent: {:?})", i, class.name, class.parent);
                println!("Properties:");
                for prop in &class.properties {
                    match &prop.value {
                        parser_code::CodeValue::String(s) => println!("  {} = \"{}\"", prop.name, s),
                        parser_code::CodeValue::Number(n) => println!("  {} = {}", prop.name, n),
                        parser_code::CodeValue::Array(arr) => {
                            println!("  {}[] = {{", prop.name);
                            for item in arr {
                                println!("    {}", item);
                            }
                            println!("  }}");
                        },
                        parser_code::CodeValue::Class(c) => {
                            println!("  {} = class {} (parent: {:?})", prop.name, c.name, c.parent);
                        }
                    }
                }
                println!();
            }
        },
        Err(errors) => {
            println!("Failed to parse file with {} errors:", errors.len());
            
            for error in errors {
                let severity = match error.severity() {
                    hemtt_workspace::reporting::Severity::Error => "ERROR",
                    hemtt_workspace::reporting::Severity::Warning => "WARNING",
                    _ => "OTHER", // Handle any other severity levels
                };
                
                println!("[{}] {}: {}", severity, error.ident(), error.message());
                
                if let Some(diagnostic) = error.diagnostic() {
                    // Just print that we have a diagnostic without accessing potentially private fields
                    println!("  Diagnostic available");
                    
                    if !diagnostic.notes.is_empty() {
                        println!("  Notes:");
                        for note in &diagnostic.notes {
                            println!("    - {}", note);
                        }
                    }
                }
            }
            
            std::process::exit(1);
        }
    }
} 