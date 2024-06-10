FROM mcr.microsoft.com/devcontainers/base:bullseye

USER root

# install dependencies
RUN apt-get update && \
    apt-get install -y libsqlite3-dev

USER vscode

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
