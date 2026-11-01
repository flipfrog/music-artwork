// Set an artwork image to music files.
use clap::{Parser};
use same_file::is_same_file;
use std::fs;
use std::fs::{create_dir_all, exists, DirEntry};
use std::process::{Command};
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
    /// Path to ffmpeg
    #[arg(short, long, default_value = "ffmpeg")]
    ffmpeg: String,
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
    let ffmpeg_path = args.ffmpeg;

    // create dir for modified files, if it does not exist.
    match exists(&dest_dir) {
        Ok(true) => {},
        Ok(false) => create_dir_all(&dest_dir)
            .with_context(|| format!("Directory creation failed for path: {dest_dir}"))?,
        _ => bail!("Directory existence check failed for path: {dest_dir}.")
    }

    // open src directory.
    let dir = fs::read_dir(&src_dir)
        .with_context(|| format!("Directory read error for path: {src_dir}"))?;

    // add artwork and output modified to dest dir, to each music files.
    let mut processed_count = 0;
    for entry_option in dir {
        let entry = entry_option.context("Directory entry acquiring error.")?;
        let os_file_name = entry.file_name();
        let Some(file_name) = os_file_name.to_str() else {
            bail!("Could not acquire file name.");
        };

        if file_name.ends_with(&extension) {
            let src_file = format!("{}/{}", &src_dir, file_name);
            let dest_file = format!("{}/{}", &dest_dir, file_name);

            let (should_skip, skip_reason) =
                check_if_skipping(&entry, &src_file, &dest_file).context("Checking to skip error.")?;

            if should_skip {
                println!("*** Skipping: {skip_reason}");
                continue;
            }

            // run ffmpeg with treated parameters.
            exec_ffmpeg(&ffmpeg_path, &src_file, &dest_file, &artwork).context("ffmpeg execution error.")?;

            println!("Done: \"{}\"", &dest_file);
            processed_count += 1;
        }
    }

    match processed_count {
        0 => println!("*** No file processed."),
        _ => println!("{} file processed.", processed_count)
    };

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
            Ok(true) => bail!("Directories of src and dest must be different."),
            Ok(false) => {},
            Err(e) => bail!("Directory comparing error {}", e)
        }
    }

    Ok(())
}

// check if skipping src_file.
fn check_if_skipping(entry: &DirEntry, src_file: &str, dest_file: &str) -> Result<(bool, String)> {
    // check existence
    if exists(dest_file)? {
        let ret = (true, String::from(format!("\"{}\" already exists.", dest_file)));
        return Ok(ret);
    }

    // check file type
    let metadata = entry.metadata()?;
    if metadata.is_dir() {
        let ret = (true, String::from(format!("\"{}\" is directory.", src_file)));
        return Ok(ret);
    }

    Ok((false, "".to_string()))
}

// execute ffmpeg.
fn exec_ffmpeg(ffmpeg_path: &str, src_file: &str, dest_file: &str, artwork: &str) -> Result<()> {
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
    let ffmpeg_output = Command::new(ffmpeg_path).args(args).output().context("could not exec ffmpeg.")?;

    if !ffmpeg_output.status.success() {
        let err_log = String::from_utf8_lossy(&ffmpeg_output.stderr);
        bail!("ffmpeg failed: {}", err_log);
    }

    Ok(())
}
