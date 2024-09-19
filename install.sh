#!/usr/bin/env sh
NO_FORMAT="\033[0m"
C_RED="\033[38;5;196m"
C_GREEN="\033[38;5;28m"

set -e

error() {
    echo "${C_RED}Unexpected error while installing PLC${NO_FORMAT}"
    rm plc_amd64.deb
    exit 1
}

main() {
    echo "${C_GREEN}Installing PLC...${NO_FORMAT}"

    curl -sSfOL https://github.com/vzalygin/plc/releases/latest/download/plc_amd64.deb || return 1
    sudo apt install -f -y ./plc_amd64.deb || return 1
    rm plc_amd64.deb || return 1

    echo "${C_GREEN}PLC has been installed successfully!${NO_FORMAT}"
}

main "$@" || error
