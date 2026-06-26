FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Install basic build tools and dependencies required for Rust (line 1) and enarx (line 2)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install build-essential curl ca-certificates libssl-dev pkg-config \
    && apt-get -y install git gcc musl-tools python3-minimal
