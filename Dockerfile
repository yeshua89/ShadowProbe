# Multi-stage Dockerfile para ShadowProbe
# Stage 1: Builder con Rust completo
FROM rust:1.85-slim-bookworm AS builder

# Instalar dependencias del sistema necesarias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Crear directorio de trabajo
WORKDIR /app

# Copiar manifests primero para cachear dependencias
COPY Cargo.toml Cargo.lock* ./
COPY crates/shadowprobe-core/Cargo.toml ./crates/shadowprobe-core/
COPY crates/shadowprobe-scanner/Cargo.toml ./crates/shadowprobe-scanner/
COPY crates/shadowprobe-ai/Cargo.toml ./crates/shadowprobe-ai/
COPY crates/shadowprobe-report/Cargo.toml ./crates/shadowprobe-report/
COPY crates/shadowprobe-cli/Cargo.toml ./crates/shadowprobe-cli/

# Crear archivos dummy para compilar dependencias
RUN mkdir -p crates/shadowprobe-core/src \
    crates/shadowprobe-scanner/src \
    crates/shadowprobe-ai/src \
    crates/shadowprobe-report/src \
    crates/shadowprobe-cli/src && \
    echo "fn main() {}" > crates/shadowprobe-cli/src/main.rs && \
    echo "pub fn dummy() {}" > crates/shadowprobe-core/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/shadowprobe-scanner/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/shadowprobe-ai/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/shadowprobe-report/src/lib.rs

# Compilar dependencias (esto se cachea)
RUN cargo build --release

# Remover archivos dummy
RUN rm -rf crates/*/src

# Copiar código fuente real
COPY crates ./crates

# Rebuild con código real (touch para forzar recompilación)
RUN find crates -name "*.rs" -exec touch {} + && \
    cargo build --release --bin shadowprobe

# Stage 2: Runtime mínimo
FROM debian:bookworm-slim

# Instalar solo las librerías runtime necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -m -u 1000 shadowprobe

WORKDIR /app

# Copiar binario desde builder
COPY --from=builder /app/target/release/shadowprobe /usr/local/bin/shadowprobe

# Crear directorio para output
RUN mkdir -p /app/output && chown shadowprobe:shadowprobe /app/output

USER shadowprobe

ENTRYPOINT ["shadowprobe"]
CMD ["--help"]

# Stage 3: Development (con hot-reload)
FROM rust:1.85-slim-bookworm AS dev

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Instalar cargo-watch para hot reload (versión compatible)
RUN cargo install cargo-watch --version 8.4.0

WORKDIR /app

# El código se montará como volumen
VOLUME ["/app"]

CMD ["cargo", "watch", "-x", "run"]
