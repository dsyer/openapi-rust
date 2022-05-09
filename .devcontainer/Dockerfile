# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.209.6/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version (use hirsuite or bionic on local arm64/Apple Silicon): hirsute, focal, bionic
ARG VARIANT="hirsute"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
# [Optional] Uncomment this section to install additional OS packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends autoconf automake libtool curl \
    make g++ unzip pkg-config bzip2 lbzip2 xz-utils

USER vscode

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

RUN curl https://get.sdkman.io | bash

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN bash -c 'source "/home/vscode/.sdkman/bin/sdkman-init.sh" && sdk install java 11.0.14-tem'

ENV JAVA_HOME=/home/vscode/.sdkman/candidates/java/current


