# Docker image for compiling the server
FROM rust:1.65.0 as builder

# Set working directory to app
WORKDIR /app

# Copying all files to our work directory
COPY . .

# Building the binary
RUN cargo build --release


# Docker image for running the server
FROM debian:bullseye-slim as runtime

ARG PORT

# Set working directory to app
WORKDIR /app

# Copying the compiled binary
COPY --from=builder /app/target/release/dinoly dinoly 
#Copying configuration
COPY config config

RUN apt-get update -y\
    && apt-get install -y --no-install-recommends openssl ca-certificates\
    && apt-get autoremove\
    && apt-get clean -y\
    && rm -rf /var/lib/apt/lists/*

# Setting environment variable to production mode
ENV APP_APPLICATION__PORT ${PORT}
ENV ENVIRONMENT production
ENV RUST_LOG debug

# Entrypoint to our binary
ENTRYPOINT ["./dinoly"]
