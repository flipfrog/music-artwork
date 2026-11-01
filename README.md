# music-artwork
A cli command for adding artwork image to music files using ffmpeg.

- The command searches sound files in directory specified.
- So it is a ffmpeg wrapper app.

## REQUIREMENT
ffmpeg command is installed and command search path reaches it.

## USAGE

`music-artwork -s <src> -d <dest> -e <extension ex: mp3> -i <artwork_file>`

Set artwork image to music files such as mp3/m4a files.

- Directory path specified by `src` assumed to contain music files.
- Directory path specified by `dest` assumed not to have same name files of `src`. If `src` had same name file which will be skipped and display warning message.
- Extension specified by `extension` is music file's extension part. For example `mp3`. This option is for avoiding modify files which is not music file.
- Artwork file specified by `artwork` is file path to image file. Such as PNG, JPG and so on. Image data will not be downsized by this command. So you specify 1MB size image file, the output file size will increase 1MB. To avoid this you can using image manipulation tool such as ImageMagic to reduce file size.

