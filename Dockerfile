# syntax=docker/dockerfile:1

FROM node:24.22.0-bookworm-slim AS node-tools
RUN npm install --global pnpm@10.34.7

FROM node-tools AS frontend
WORKDIR /app/src/web
COPY src/web/package.json src/web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY src/web/ ./
ARG VERSION=dev
RUN VITE_APP_VERSION="$VERSION" pnpm build

FROM rust:1.100-bookworm AS rust-tools

FROM rust-tools AS dev
COPY --from=node-tools /usr/local/ /usr/local/
RUN rustup component add rustfmt clippy
RUN apt-get update \
    && apt-get install -y --no-install-recommends nano zsh \
    && rm -rf /var/lib/apt/lists/*
ARG UID=1000
ARG GID=1000
RUN groupadd --gid "$GID" dev \
    && useradd --uid "$UID" --gid "$GID" --create-home --shell /bin/zsh dev \
    && install -d -o dev -g dev /workspace /home/dev/.cargo /cache
ENV CARGO_HOME=/home/dev/.cargo
ENV LANG=C.UTF-8
ENV TERM=xterm-256color
WORKDIR /workspace
USER dev
RUN install -m 600 /dev/null /cache/.zsh_history \
    && sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended \
    && git clone --depth 1 https://github.com/zsh-users/zsh-autosuggestions.git "$HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions"
COPY --chown=dev:dev zshrc /home/dev/.zshrc

FROM rust-tools AS build
ARG VERSION=dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/core/ src/core/
COPY src/server/ src/server/
COPY --from=frontend /app/src/web/dist/ src/web/dist/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    APP_VERSION="$VERSION" cargo build --release --locked --features embed-web \
    && install -D target/release/template-rust-web-app /out/template-rust-web-app

FROM debian:bookworm-slim AS runtime
ARG VERSION=dev
LABEL org.opencontainers.image.version="$VERSION"
COPY --from=build /out/template-rust-web-app /usr/local/bin/template-rust-web-app
ENV LISTEN_ADDR=0.0.0.0:8080
EXPOSE 8080
USER 10001:10001
ENTRYPOINT ["template-rust-web-app"]
