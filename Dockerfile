# syntax=docker/dockerfile:1

########################## BUILD IMAGE  ##########################
FROM docker.io/library/rust:1.72.0-bookworm as build

# Get the front-end
RUN wget -c https://github.com/Timshel/oidc_web_builds/releases/download/v2023.8.2/oidc_button_web_vault-v2023.8.2-1.tar.gz -O - | tar -xz

# Build time options to avoid dpkg warnings and help with reproducible builds.
ENV DEBIAN_FRONTEND=noninteractive \
    LANG=C.UTF-8 \
    TZ=UTC \
    TERM=xterm-256color \
    CARGO_HOME="/root/.cargo" \
    REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    USER="root"

# Create CARGO_HOME folder and don't download rust docs
RUN mkdir -pv "${CARGO_HOME}" \
    && rustup set profile minimal

# Install build dependencies
RUN apt-get update \
    && apt-get install -y \
        --no-install-recommends \
        libmariadb-dev \
        libpq-dev

# Creates a dummy project used to grab dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app

# Copies over *only* your manifests and build files
COPY ./Cargo.* ./
COPY ./rust-toolchain.toml ./rust-toolchain.toml
COPY ./build.rs ./build.rs


# Configure the DB ARG as late as possible to not invalidate the cached layers above
ARG DB=sqlite,mysql,postgresql

# Builds your dependencies and removes the
# dummy project, except the target folder
# This folder contains the compiled dependencies
RUN cargo build --features ${DB} --release \
    && find . -not -path "./target*" -delete

# Copies the complete project
# To avoid copying unneeded files, use .dockerignore
COPY . .

# Make sure that we actually build the project
RUN touch src/main.rs

# Builds again, this time it'll just be
# your actual source files being built
RUN cargo build --features ${DB} --release

######################## RUNTIME IMAGE  ########################
# Create a new stage with a minimal image
# because we already have a binary built
FROM docker.io/library/debian:bookworm-slim

ENV ROCKET_PROFILE="release" \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=80


# Create data folder and Install needed libraries
RUN mkdir /data \
    && apt-get update && apt-get install -y \
    --no-install-recommends \
    ca-certificates \
    curl \
    libmariadb-dev-compat \
    libpq5 \
    openssl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*


VOLUME /data
EXPOSE 80
EXPOSE 3012

# Copies the files from the context (Rocket.toml file and web-vault)
# and the binary from the "build" stage to the current stage
WORKDIR /
COPY --from=build /web-vault ./web-vault
COPY --from=build /app/target/release/vaultwarden .

COPY docker/healthcheck.sh /healthcheck.sh
COPY docker/start.sh /start.sh

HEALTHCHECK --interval=60s --timeout=10s CMD ["/healthcheck.sh"]

CMD ["/start.sh"]
