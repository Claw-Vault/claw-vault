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

ARG BACKEND_URL
EXPOSE 3000

# Start the BUN server
CMD ["bun", "--bun", "run", "start"]
