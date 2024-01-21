FROM rust:1.75.0-alpine3.19 as builder

WORKDIR /usr/src/app

COPY . .

RUN apk add --no-cache musl-dev perl-utils gcc make

RUN cargo install --path .

ARG DATABASE_URL

FROM alpine:3.19

COPY --from=builder /usr/local/cargo/bin/claw-vault /usr/local/bin/claw-vault

EXPOSE 3000

CMD [ "claw-vault" ]
