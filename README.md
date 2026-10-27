# music-artwork
cli for adding artwork image to music files using ffmpeg.

## REQUIREMENT
- ffmpeg command is installed and command search path includes it.

## USAGE

`music-artwork <src_dir> <dest_dir> <extension ex: mp3> <artwork_file>`

Set artwork image to music files such as mp3/m4a files.

- Directory path specified by `src_dir` assumed to contain music files.
- Directory path specified by `dest_dir` assumed not to have same name files of `src_dir`. If `src_dir` had same name file, this command will stop the process and display error message.
- Extension specified by `extension` is music file's extension part. For example `mp3`. This option is for avoiding modify files which is not music file.
- Artwork file specified by `artwork` is file path to image file. Such as PNG, JPG and so on. Image data will not be downsized by this command. So you specify 1MB size image file, the output file size will increase 1MB. To avoid this you can using image manipulation tool such as ImageMagic to reduce file size.

