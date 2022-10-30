# enigma
An implementation of the Enigma Machine written in rust
> The Enigma machine is a cipher device developed and used in the early- to mid-20th century to protect commercial, diplomatic, and military communication. It was employed extensively by Nazi Germany during World War II, in all branches of the German military. The Enigma machine was considered so secure that it was used to encipher the most top-secret messages.

### Installation
- [Install Rust](https://www.rust-lang.org/)  
- run `cargo install libenigma`  
- run `libenigma -h` for usage instructions  

### Tests
run `cargo test`

### Benchmarks
run `cargo bench`

# Usage
- -i Input file: Path to input text file
- -c Config file: Path to config file. If a config file does not exist at this path, a config file will be generated a written to this path
- -o Output file: Path to output ciphertext
- -h Help: Help information- Output information on usage of CLI
- -v Version: Output version information
