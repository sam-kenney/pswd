# pswd
Password generation CLI tool.

## Install
Install [Cargo](https://www.rust-lang.org/tools/install)
```sh
git clone git@github.com:sam-kenney/pswd.git
cargo install --path pswd
```

## Usage
```sh
pswd <length> <OPTIONS>
```

### OPTIONS
Each flag includes characters to the valid characters to choose from.
If no flag is specified, `-a` is used. Specifying any other CHARSET OPTION 
FLAGS with `-a` conflicts with `-a` and is not allowed.

FLAGS
- `-a`: All ASCII characters
- `-u`: Uppercase ASCII characters
- `-l`: Lowercase ASCII characters
- `-n`: Numerical ASCII characters
- `-s`: ASCII symbol characters

The generated password will be copied into your clipboard. If you are experiencing
issues with this, you may use the `-p` flag to print the generated password to STDOUT.
The PRINT FLAG defaults to `false`.

You can also use the `-d` to print any debug information such as specific errors when
copying to the clipboard.
