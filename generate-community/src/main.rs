use std::{fs, path::PathBuf};

use clap::Parser;
use generate_community::collect_member_files;

/// Collects the TOML files
/// containing info pertaining
/// to community and organization
/// members into a single file
/// for easier parsing by
/// the template pages.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing the
    /// TOML member files.
    #[arg(short, long)]
    input_folder: PathBuf,
    /// File where the member files
    /// data will be collected
    /// for output.
    #[arg(short, long)]
    output_file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("Collecting member files into a single file . . .");
    let member_file = collect_member_files(&args.input_folder)?;
    println!(
        "Outputting members file to {} . . .",
        args.output_file.to_string_lossy()
    );
    fs::write(args.output_file, member_file)?;

    Ok(())
}
