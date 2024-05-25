## dirscover

Randomly discover files on a wide directory structure.
root_directory - starting directory for traversal
N - number of files to randomly discover
exclusions - a comma-delimited list of exclusions (supports wildcards)

# usage

`cargo run root_directory N exclusions`

# example

`cargo run ~/data/ 25 .bash_history,*.rar,*.exe,*.js,*.zip,*.xml,*.py,*.jpg,*.html`

