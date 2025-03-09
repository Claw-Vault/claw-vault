FROM rust:1.85.0-alpine3.21 as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apk add --no-cache musl-dev perl-utils make curl make pkgconf openssl-dev openssl-libs-static

ARG DATABASE_URL
ARG PORT

# Build the application
RUN cargo test
RUN cargo install --locked --path .

# Start a new, final image
FROM alpine:3.21

# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/claw-vault /usr/local/bin/claw-vault
COPY --from=builder /usr/src/app/lib-domain/migrations /usr/local/bin/migrations

EXPOSE 8080

# Run the binary
CMD [ "claw-vault" ]
