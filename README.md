# rust_cat

**Build the project in release mode:**
```bash
    cargo build --release   # or: cargo build -r
```

**Usage:**
```bash
    target/release/rust_cat [-AbeEnstTv] [file_path]
```

*If file_path is omitted or replaced with -, the program reads from standard input.*

**Options:**

|Option|Usage|
|--|--|
|-A|Equivalent to -vET.|
|-b|Number non-blank output lines, overrides -n.|
|-e|Equivalent to -vE.|
|-E|Display a $ at the end of each line.|
|-n|Number all output lines.|
|-s|Suppress repeated empty output lines.|
|-t|Equivalent to -vT.|
|-T|Display TAB characters as ^I.|
|-v|Use ^ and M- notation, except for LFD and TAB.|

*Note: Only short options are currently supported. Long options may be added later.*

**Default behavior:**

If no options are supplied, rust_cat behaves like the traditional cat utility: It waits for user input from standard input and echoes
it back to standard output until an EOF (End Of File) signal is received (usually by pressing Ctrl+D on Unix-like systems).

⚠️ ***Though I spent a good amount of time optimizing this project, it has been created for learning purposes and is in no way intended to be a replacement
for GNU coreutils version of cat*** ⚠️

