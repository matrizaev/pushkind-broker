FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends libzmq3-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
