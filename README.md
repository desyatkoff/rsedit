# Rsedit

```
 ____  ____  _____ ____ ___ _____ 
|  _ \/ ___|| ____|  _ \_ _|_   _|
| |_) \___ \|  _| | | | | |  | |  
|  _ < ___) | |___| |_| | |  | |  
|_| \_\____/|_____|____/___| |_|  
```

## Description

**Rsedit** (stands for **R**u**s**t **edit**or) is an app to quickly edit your text files with text-based user interface, you can launch it right in terminal, even through SSH connection!

## Features

* Basic interaction with text
    + Read
    + Write
* Keyboard shortcuts
    + `Control` + `S` -> Save
    + `Control` + `Q` -> Quit
* Status bar
    + File name
    + Modification indicator
    + Cursor position
    + Total lines count

## Installation

1. **Clone the repository**
    ```Shell
    $ git clone https://github.com/desyatkoff/rsedit.git
    ```
2. **Go to the repository directory**
    ```Shell
    $ cd rsedit/
    ```
3. **Compile the Rust project**
    ```Shell
    $ cargo build --release
    ```
4. **Copy Rsedit binary to the `/bin/` directory**
    ```Shell
    $ cp ./target/release/rsedit /bin/
    ```

## Usage

Firstly, you have to [install Rsedit](#installation) and then enter this command:
```Shell
$ rsedit
```
After that, you will be moved to Rsedit screen so you can start writing your text. If you want to open an existing file, you should launch Rsedit with an argument:
```Shell
$ rsedit /path/to/file.txt
```
Or you can use relative path (if your file is near to your current location in terminal):
```Shell
$ rsedit file.txt
```
Okay. But how to save all the changes? No problem, just `Control` + `S` \
How to quit this horrible thing? `Control` + `Q`
