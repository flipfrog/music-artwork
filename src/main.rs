// Set an artwork image to music files.
use clap::{Parser};
use same_file::is_same_file;
use std::fs;
use std::fs::{create_dir_all, exists, DirEntry};
use std::process::{exit, Command};
use anyhow::{bail, Context, Result};

// declaration of command arguments.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input directory
    #[arg(short, long)]
    src: String,
    /// Output directory (if the directory does not exist, it will be created)
    #[arg(short, long)]
    dest: String,
    /// Extension of music files such as mp3, aac, flac, etc...
    #[arg(short, long)]
    ext: String,
    /// Artwork image file path
    #[arg(short, long)]
    img: String,
}

fn main() -> Result<()> {
    // analyze args.
    let args = Args::parse();

    // check commandline arguments.
    check_arg_values(&args).context("Argument error.")?;

    let src_dir = args.src;
    let dest_dir = args.dest;
    let extension = format!(".{}", &args.ext);
    let artwork = args.img;

    // create dir for modified files, if it does not exist.
    let is_exist = exists(&dest_dir)
        .with_context(|| format!("Directory existence check failed for path: {dest_dir}."))?;

    if !is_exist {
        create_dir_all(&dest_dir)
            .with_context(|| format!("Directory creation failed for path: {dest_dir}"))?;
    }

    // open src directory.
    let dir = fs::read_dir(&src_dir)
        .with_context(|| format!("Directory read error for path: {src_dir}"))?;

    // add artwork and output modified to dest dir, to each music files.
    let mut processed_count = 0;
    for entry_option in dir {
        let entry = entry_option.context("Directory entry acquiring error.")?;
        let os_file_name = entry.file_name();
        let file_name = if let Some(file_name) = os_file_name.to_str() {
            file_name
        } else {
            println!("Could not acquire file name.");
            exit(1);
        };

        if file_name.ends_with(&extension) {
            let src_file = format!("{}/{}", &src_dir, file_name);
            let dest_file = format!("{}/{}", &dest_dir, file_name);

            let (should_skip, skip_reason) =
                check_if_skipping(&entry, &dest_file).context("Checking to skip error.")?;

            if should_skip {
                println!("{skip_reason}");
                continue;
            }

            // run ffmpeg with treated parameters.
            exec_ffmpeg(&src_file, &dest_file, &artwork).context("ffmpeg execution error.")?;

            println!("Done: \"{}\"", &dest_file);
            processed_count += 1;
        }
    }

    if processed_count == 0 {
        println!("*** No file processed.");
    } else {
        println!("{} file processed.", processed_count);
    }

    Ok(())
}

// check argument values.
fn check_arg_values(args: &Args) -> Result<()> {
    if !exists(&args.src)? {
        bail!("No such src directory: {}", args.src);
    }

    // assuming src and dest directories are different.
    if exists(&args.dest)? {
        match is_same_file(&args.src, &args.dest) {
            Ok(is_same) => if is_same {
                bail!("Directories of src and dest must be different.");
            },
            Err(e) => bail!("Directory comparing error {}", e)
        }
    }

    Ok(())
}

// check if skipping src_file is directory.
fn check_if_skipping(entry: &DirEntry, dest_file: &str) -> Result<(bool, String)> {
    // check existence
    if exists(dest_file)? {
        let ret = (true, String::from(format!("*** Skipping, \"{}\" already exists.", dest_file)));
        return Ok(ret);
    }

    // check file type
    let metadata = entry.metadata()?;
    if metadata.is_dir() {
        let ret = (true, String::from(format!("*** Skipping, \"{}\" is directory.", dest_file)));
        return Ok(ret);
    }

    Ok((false, "".to_string()))
}

// execute ffmpeg.
fn exec_ffmpeg(src_file: &str, dest_file: &str, artwork: &str) -> Result<()> {
    let args = vec![
        "-i", src_file,
        "-i", artwork,
        "-map", "0:a",
        "-map", "1:v",
        "-c", "copy",
        "-disposition:1", "attached_pic",
        "-id3v2_version", "3",
        dest_file
    ];

    // ffmpeg -i in.mp3 -i <img-file-path> -map 0:a -map 1:v -c copy -disposition:1 attached_pic -id3v2_version 3 out.mp3
    let ffmpeg_output = Command::new("ffmpeg").args(args).output().context("could not exec ffmpeg.")?;

    if !ffmpeg_output.status.success() {
        let err_log = String::from_utf8_lossy(&ffmpeg_output.stderr);
        bail!("ffmpeg failed: {}", err_log);
    }

    Ok(())
}
