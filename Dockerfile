FROM oven/bun:1.1 AS builder

WORKDIR /app
COPY . .

ARG BACKEND_URL

ENV NODE_ENV=production
RUN bun --bun install
RUN bun --bun run build

# Create a smaller image for running the application
FROM oven/bun:1.1

COPY --from=builder /app/build .

EXPOSE 3000

# Start the BUN server
ENV PROTOCOL_HEADER=x-forwarded-proto
ENV HOST_HEADER=x-forwarded-host
CMD ["bun", "--bun", "run", "start"]
