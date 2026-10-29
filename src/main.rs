// Set an artwork image to music files.
use std::fs;
use std::fs::{create_dir_all, exists};
use std::process::{Command};
use clap::{Parser};

// declaration of command arguments.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input directory
    #[arg(short, long)]
    src: String,
    /// output directory (if the directory does not exist, it will be created)
    #[arg(short, long)]
    dest: String,
    /// extension of music files such as mp3, flac, etc...
    #[arg(short, long, default_value = "mp3")]
    ext: String,
    /// artwork image file path
    #[arg(short, long)]
    img: String,
}

fn main() {
    // analyze args.
    let args = Args::parse();

    let src_dir = &args.src;
    let dir = fs::read_dir(src_dir).expect("dir opening error.");
    let dest_dir = &args.dest;
    let extension = format!(".{}", &args.ext);
    let artwork = &args.img;

    // create dir for modified files, if it does not exist.
    if !exists(dest_dir).unwrap() {
        create_dir_all(dest_dir).unwrap();
    }

    // add artwork and output modified to dest dir, to each music files.
    for entry_option in dir {
        let entry = entry_option.expect("entry taking error.");
        let os_file_name = entry.file_name();
        let file_name = os_file_name.to_str().unwrap();

        if file_name.ends_with(&extension) {
            let src_file = &format!("{}/{}", src_dir, file_name);
            let dest_file = &format!("{}/{}", dest_dir, file_name);

            if exists(dest_file).unwrap() {
                println!("skipping, file already exists: {}", dest_file);
                continue;
            }

            // ffmpeg -i in.mp3 -i <img-file-path> -map 0:a -map 1:v -c copy -disposition:1 attached_pic -id3v2_version 3 out.mp3
            let _ffmpeg_output = Command::new("ffmpeg")
                .args([
                    "-i", src_file,
                    "-i", artwork,
                    "-map", "0:a",
                    "-map", "1:v",
                    "-c", "copy",
                    "-disposition:1", "attached_pic",
                    "-id3v2_version", "3",
                    dest_file
                ])
                .output()
                .expect("failed executing ffmpeg");

            println!("succeeded {}", dest_file);
            // println!("{}", String::from_utf8_lossy(&_ffmpeg_output.stderr))
        }
    }
}
