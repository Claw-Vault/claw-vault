FROM oven/bun:1.2.4 AS builder

WORKDIR /app
COPY . .

ARG BACKEND_URL
ARG PROD_ORIGIN

ENV NODE_ENV=production
RUN bun install
RUN bun run build

# Create a smaller image for running the application
FROM oven/bun:1.2.4

COPY --from=builder /app/build .

EXPOSE 3000

# Start the BUN server
ARG PROD_ORIGIN
ENV ORIGIN=$PROD_ORIGIN
ENV NODE_ENV=production
ENTRYPOINT [ "bun", "build/index.js" ]
