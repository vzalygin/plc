#!/bin/bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 14 all
sudo apt-get install zlib1g-dev # fixes linking errs: https://stackoverflow.com/questions/3373995/usr-bin-ld-cannot-find-lz