apt update && apt upgrade -y && \
apt install -y curl wget git vim build-essential && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source "$HOME/.cargo/env" && \
rustup toolchain add 1.77.1 && \
rustup default 1.77.1 && \
mkdir ~/repos && \
cd ~/repos && \
git clone git@github.com:flucium/flatshell.git && \
ls