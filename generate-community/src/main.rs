use std::{fs, path::PathBuf};

use clap::Parser;
use generate_community::{collect_member_files, copy_profile_pictures};

/// Collects the TOML files
/// containing info pertaining
/// to community and organization
/// members into a single file
/// for easier parsing by
/// the template pages.
///
/// Also copies the relevant profile pictures
/// to the output folder.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing the
    /// TOML member files.
    #[arg(short, long)]
    input_folder: PathBuf,
    /// Name of file the member files
    /// data will be collected
    /// for output.
    #[arg(short, long)]
    output_file: PathBuf,
    /// Path to the pictures output
    /// folder.
    #[arg(long)]
    pictures_output_folder: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("Collecting member files into a single file . . .");
    let member_file = collect_member_files(&args.input_folder)?;

    println!(
        "Outputting members file to {} . . .",
        args.output_file.to_string_lossy()
    );
    fs::write(&args.output_file, member_file)?;

    println!(
        "Copying members' profile pictures to {} . . .",
        args.pictures_output_folder.to_string_lossy()
    );
    copy_profile_pictures(&args.input_folder, &args.pictures_output_folder)?;

    Ok(())
}
