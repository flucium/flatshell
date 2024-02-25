FROM ubuntu:22.04
RUN apt update && apt upgrade -y && \
apt install -y curl git build-essential && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source "$HOME/.cargo/env" && \
mkdir ~/repos && cd ~/repos && \
git clone git@github.com:flucium/flatshell.git && \
ls