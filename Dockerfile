FROM rust:1.76.0-alpine3.19 as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apk add --no-cache musl-dev perl-utils make

ARG DATABASE_URL
ENV TEMPLATE_DIR=templates
ENV ASSETS_DIR=assets
ARG PORT

# Build the application
RUN cargo test --release
RUN cargo install --path .

# Start a new, final image
FROM alpine:3.19

# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/claw-vault /usr/local/bin/claw-vault
COPY --from=builder /usr/src/app/templates /usr/local/bin/templates
COPY --from=builder /usr/src/app/assets /usr/local/bin/assets

EXPOSE 3000

# Run the binary
CMD [ "claw-vault" ]
