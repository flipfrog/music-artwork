// 楽曲ファイルのアートワークを設定する
// ffmpeg -i in.mp3 -i <img-file-path> -map 0:a -map 1:v -c copy -disposition:1 attached_pic -id3v2_version 3 out.mp3
use std::fs;
use std::fs::{create_dir_all, exists};
use std::process::{exit, Command};

fn main() {
    // analyze args.
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 5 {
        println!("less arguments.");
        println!("Usage: {} <src_dir> <dest_dir> <extension ex: mp3> <artwork_file>", &args[0]);
        exit(1);
    }

    let dir = fs::read_dir(&args[1]).expect("dir opening error.");
    let dest_dir = &args[2];
    let extension = format!(".{}", &args[3]);
    let artwork = &args[4];

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
            let src_file = format!("{}/{}", &args[1], &file_name);
            let dest_file = format!("{}/new_{}", &dest_dir, &file_name);

            if exists(&dest_file).unwrap() {
                println!("skipping, file already exists: {}", &dest_file);
                continue;
            }

            let _ffmpeg_output = Command::new("ffmpeg")
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

            println!("succeeded {}", dest_file);
            // println!("{}", String::from_utf8_lossy(&_ffmpeg_output.stderr))
        }
    }
}
