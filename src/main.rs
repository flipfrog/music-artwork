// Set an artwork image to music files.
use clap::{Parser};
use same_file::is_same_file;
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, exists, DirEntry};
use std::process::{exit, Command};

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

fn main() {
    // analyze args.
    let args = Args::parse();

    // check commandline arguments.
    check_arg_values(&args).unwrap_or_else(|e| {
        println!("Argument check error: {e}");
        exit(1);
    });

    let src_dir = args.src;
    let dest_dir = args.dest;
    let extension = format!(".{}", &args.ext);
    let artwork = args.img;

    // create dir for modified files, if it does not exist.
    let is_exist = exists(&dest_dir).unwrap_or_else(|e| {
        println!("Destination directory existence check error: {e}");
        exit(1);
    });

    if !is_exist {
        create_dir_all(&dest_dir).unwrap_or_else(|e| {
            println!("Destination directory creation error: {e}");
            exit(1);
        });
    }

    // open src directory.
    let dir = fs::read_dir(&src_dir).unwrap_or_else(|e| {
        println!("Src directory opening error: {e}");
        exit(1);
    });

    // add artwork and output modified to dest dir, to each music files.
    let mut processed_count = 0;
    for entry_option in dir {
        let entry = entry_option.unwrap_or_else(|e| {
            println!("Directory entry acquiring error: {e}");
            exit(1);
        });
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

            let (should_skip, skip_reason) = check_if_skipping(&entry, &dest_file).unwrap_or_else(|e| {
                println!("Checking to skip error: {e}");
                exit(1);
            });
            if should_skip {
                println!("{skip_reason}");
                continue;
            }

            // run ffmpeg with treated parameters.
            exec_ffmpeg(&src_file, &dest_file, &artwork).unwrap_or_else(|e| {
                println!("ffmpeg execution error: {e}");
                exit(1);
            });

            println!("Done: \"{}\"", &dest_file);
            processed_count += 1;
        }
    }

    if processed_count == 0 {
        println!("*** No file processed.");
    } else {
        println!("{} file processed.", processed_count);
    }
}

// check argument values.
fn check_arg_values(args: &Args) -> Result<(), Box<dyn Error>> {
    if !exists(&args.src)? {
        return Err(format!("No such src directory: {}", args.src).into());
    }

    // assuming src and dest directories are different.
    if exists(&args.dest)? {
        match is_same_file(&args.src, &args.dest) {
            Ok(is_same) => if is_same {
                return Err("Directories of src and dest must be different.".into());
            },
            Err(e) => {
                return Err(format!("Directory comparing error {}", e).into());
            }
        }
    }

    Ok(())
}

// check if skipping src_file is directory.
fn check_if_skipping(entry: &DirEntry, dest_file: &str) -> Result<(bool, String), Box<dyn Error>> {
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
fn exec_ffmpeg(src_file: &str, dest_file: &str, artwork: &str) -> Result<(), Box<dyn Error>> {
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
    let ffmpeg_output = match Command::new("ffmpeg").args(args).output() {
        Ok(output) => output,
        Err(e) => {
            return Err(format!("could not exec ffmpeg: {e}").into())
        }
    };

    if !ffmpeg_output.status.success() {
        let err_log = String::from_utf8_lossy(&ffmpeg_output.stderr);
        return Err(format!("ffmpeg failed: {}", err_log).into());
    }

    Ok(())
}
