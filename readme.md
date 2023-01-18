# Aria2 Magnet Extract

This is a simple command line tool for extracting the magnet link from an Aria2 control file. Use it to recover magnet
links when aria2c download was interrupted, but the original link was lost. It is written for
educational purposes and to demonstrate the usage of Rust programming language.

## Usage

```shell
aria2_magnet_extract <path-to-aria2-file>
```

## Output

```shell
magnet:?xt=urn:btih:<hash>
```

## Build and Run

You can use `cargo run -- <path-to-aria2-file>` to build and run the project.

## Note

* This tool only works with files that have a '.aria2' file extension.
* This project is for educational purpose, and similar functionality can be achieved by using the Linux command
  ```shell
  url=$(echo "magnet:?xt=urn:btih:$(xxd -p -seek 10 -l 20 'test.aria2')")
  aria2c -c $url
  ```
* For more detailed information about the '.aria2' file format, please refer to the official documentation
  at https://aria2.github.io/manual/en/html/technical-notes.html