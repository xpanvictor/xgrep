# XGREP

## Description
Xgrep is an implementation of the famous GREP in rust. GREP which means
Globally search a Regular Expression and Print is used to search the contents
of a file for a regular expression. 
This project was built as a step for mastering the RUST programming book. 
A bigger version has been built by a fellow Rustacean by the name *Andrew Gallant*
in his project called RipGrep. 

## Usage
```bash
git clone git@github.com:xpanvictor/xgrep.git
cd xgrep
cargo run query filepath.txt
```
where 
query: a string to search for in
filepath: a path leading to file to search for

# Options
1. To ignore the search case:
- Linux: \
`IGNORE_CASE=1 cargo run query filepath`
- Powershell: \
`Env:IGNORE_CASE=1; cargo run to filepath.txt`

2. To save search result to a file \
`cargo run query search_filepath.txt > output_filepath.txt`

## Pending Updates
1. Make xgrep display line number with the line of search results.

## Contribution guidelines
1. Welcome to xgrep. The project is just a learning tool but anyone is free to contribute.
2. You can contribute via: 
- Making issues on [github issues](https://github.com/xpanvictor/xgrep/issues)
- Code contribution by submitting a pull request.
- Documentation
3. Rust joy is needed :).