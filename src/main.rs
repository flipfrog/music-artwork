// Set an artwork image to music files.
use clap::{Parser};
use same_file::is_same_file;
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, exists};
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
    check_arg_values(&args).unwrap();

    let src_dir = args.src;
    let dest_dir = args.dest;
    let extension = format!(".{}", &args.ext);
    let artwork = args.img;

    // create dir for modified files, if it does not exist.
    if !exists(&dest_dir).unwrap() {
        create_dir_all(&dest_dir).unwrap();
    }

    let dir = fs::read_dir(&src_dir).expect("dir opening error.");

    // add artwork and output modified to dest dir, to each music files.
    let mut processed_count = 0;
    for entry_option in dir {
        let entry = entry_option.expect("entry taking error.");
        let os_file_name = entry.file_name();
        let file_name = os_file_name.to_str().unwrap();

        if file_name.ends_with(&extension) {
            let src_file = format!("{}/{}", &src_dir, file_name);
            let dest_file = format!("{}/{}", &dest_dir, file_name);

            if exists(&dest_file).unwrap() {
                println!("skipping, file already exists: {}", dest_file);
                continue;
            }

            // ffmpeg -i in.mp3 -i <img-file-path> -map 0:a -map 1:v -c copy -disposition:1 attached_pic -id3v2_version 3 out.mp3
            let ffmpeg_output = Command::new("ffmpeg")
                .args([
                    "-i", &src_file,
                    "-i", &artwork,
                    "-map", "0:a",
                    "-map", "1:v",
                    "-c", "copy",
                    "-disposition:1", "attached_pic",
                    "-id3v2_version", "3",
                    &dest_file
                ])
                .output()
                .expect("failed executing ffmpeg");

            if !ffmpeg_output.status.success() {
                println!("ffmpeg failure.");
                exit(1);
            }

            println!("Succeeded: \"{}\"", &dest_file);
            processed_count += 1;
            // println!("ffmpeg log: {}", String::from_utf8_lossy(&_ffmpeg_output.stderr))
        }
    }

    if processed_count == 0 {
        println!("No sound file processed.");
    }
}

// check argument values.
fn check_arg_values(args: &Args) -> Result<(), Box<dyn Error>> {
    if !exists(&args.src)? {
        println!("No such src directory: {}", args.src);
        exit(2);
    }

    // assuming src and dest directories are different.
    if exists(&args.dest)? {
        match is_same_file(&args.src, &args.dest) {
            Ok(true) => {
                println!("Directories of src and dest must be different.");
                exit(1);
            },
            Ok(false) => {},
            Err(e) => {
                println!("Directory comparing error {}", e);
                exit(1);
            }
        }
    }

    Ok(())
}