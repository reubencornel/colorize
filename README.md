# colorize
A simple program to colorize input text.

```
USAGE:
    colorize [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --backgroundColor <BACKGROUND_COLOR>    Sets the background color
    -f, --foregroundColor <FOREGROUND_COLOR>    Sets the foreground color
    -i, --inputString <INPUT_STRING>            Sets the string to be colorized. If this is not set, the program tries
                                                to read a string from stdin

```

## Usage

```echo "Hello World!" | colorize -f red```
