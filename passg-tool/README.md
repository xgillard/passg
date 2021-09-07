# PassG-tool
PassGen is a simple command line tool to generate pseudo-random passwords
matching a desired set of (simple constraints)

## Usage
```
passg 0.2.0
PassGen is a tool that lets you generate pseudo-random passwords from the command line

USAGE:
    passg [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alpha <alpha>        What kind of alphabetic characters do you want to allow ? (all = 'all', none = 'none',
                               easily distinguished = 'dist' [eg removes O vs 0], lower case = 'lower', upper case =
                               'upper') [default: dist]
    -d, --digit <digit>        What kind of numeric characters do you want to allow ? (all = 'all', none = 'none',
                               easily distinguished = 'dist' [eg removes O vs 0]) [default: dist]
    -l, --length <length>      The length of the password [default: 20]
    -s, --special <special>    What kind of special characters do you want to allow ? (all = 'all', none = 'none', the
                               most common ones = 'basic') [default: basic]
```
