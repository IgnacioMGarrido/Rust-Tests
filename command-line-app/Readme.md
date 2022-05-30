# Command Line App

Simple command app written in Rust. When you run the app you can write into the command line a key and a value and it will drop it into a file called "cl.db".

To run the program simply run ```cargo run -- {key} {value}``` then you will write the key-value pair as you wrote it and also in upper case:

Example:

```
cargo run -- Hello World
    Compiling command-line-app v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
    Running `target\debug\command-line-app.exe Hello World`
cat cl.db  
Hello   World
HELLO   World
```
