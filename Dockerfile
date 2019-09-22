FROM ubuntu

RUN apt-get update \
    && apt-get install -y curl vim \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /rustup.sh \
    && chmod 755 /rustup.sh \
    && /rustup.sh -y \
    && echo 'export PATH="$HOME/.cargo/bin:$PATH"' > "$HOME/.bashrc"

ENTRYPOINT bash
