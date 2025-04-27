# Rsedit

```
 ____  ____  _____ ____ ___ _____ 
|  _ \/ ___|| ____|  _ \_ _|_   _|
| |_) \___ \|  _| | | | | |  | |  
|  _ < ___) | |___| |_| | |  | |  
|_| \_\____/|_____|____/___| |_|  
```

## Description

**Rsedit** (stands for **R**u**s**t **edit**or) is a great TUI tool to quickly edit your text files. You can launch it right in terminal, even through SSH connection!

## Features

* Basic interaction with text
    + Read
    + Write
    + Copy
    + Paste
* Keyboard shortcuts
    + `Control` + `F` -> Search
        - `Down` or `Right` -> Next
        - `Up` or `Left` -> Previous
    + `Control` + `S` -> Save
    + `Control` + `Q` -> Quit
    + `Control` + `Shift` + `C` -> Copy
    + `Control` + `Shift` + `V` -> Paste
* Dynamic status bar
    + File name
    + Modification indicator
    + Cursor position
    + Total lines count
* Dynamic hint bar
* Command bar
    + Search
        - Next
        - Previous
    + Save as

## Installation

### Easy method

1. **Clone the repository**
    ```Shell
    $ git clone https://github.com/desyatkoff/rsedit.git
    ```
2. **Go to the repository directory**
    ```Shell
    $ cd ./rsedit/
    ```
3. **Launch `install.sh` script**
    ```Shell
    $ sh ./install.sh
    ```

### Normal  method

1. **Clone the repository**
    ```Shell
    $ git clone https://github.com/desyatkoff/rsedit.git
    ```
2. **Go to the repository directory**
    ```Shell
    $ cd ./rsedit/
    ```
3. **Compile the Rust project**
    ```Shell
    $ cargo build --release
    ```
4. **Copy Rsedit binary to the `/bin/` directory**
    ```Shell
    $ sudo cp ./target/release/rsedit /bin/
    ```
5. **Test Rsedit using `example.txt` that included to the repository**
    ```Shell
    $ rsedit example.txt
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

If you forgot to create a file to edit with Rsedit, you still can `Control` + `S` and the command bar will appear asking you a wanted file name (simply a "save as" feature)
