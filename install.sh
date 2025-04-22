#!/usr/bin/env bash

########################################
#                                      #
#   ____  ____  _____ ____ ___ _____   #
#  |  _ \/ ___|| ____|  _ \_ _|_   _|  #
#  | |_) \___ \|  _| | | | | |  | |    #
#  |  _ < ___) | |___| |_| | |  | |    #
#  |_| \_\____/|_____|____/___| |_|    #
#                                      #
#  -------- RSEDIT INSTALLER --------  #
#                                      #
########################################

# 1. Clean files (in case if already installed)

cargo clean

sudo rm -v /bin/rsedit

# 2. Compile the Rust project

cargo build --release

# 3. Copy binary to the `/bin/` directory

sudo cp -v \
    ./target/release/rsedit \
    /bin/

# 4. After installation

rsedit example.txt    # `example.txt` file is included into the repository (just if you want to quickly test Rsedit)

# Success!
# Enjoy your new *blazingly fast* text editor right in terminal
