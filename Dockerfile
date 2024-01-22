FROM rust:1.75.0-alpine3.19 as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apk add --no-cache musl-dev perl-utils make

# Build the application
RUN cargo test --release -- --nocapture --exact
RUN cargo install --path .

ARG DATABASE_URL
ARG PORT

# Start a new, final image
FROM alpine:3.19

# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/claw-vault /usr/local/bin/claw-vault
COPY --from=builder /usr/src/app/templates /usr/local/bin/templates

EXPOSE 3000

# Run the binary
CMD [ "claw-vault" ]
