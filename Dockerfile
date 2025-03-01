FROM oven/bun:1.1 AS builder

WORKDIR /app
COPY . .

ARG BACKEND_URL
ARG PROD_ORIGIN

ENV ORIGIN=$PROD_ORIGIN
ENV NODE_ENV=production
RUN bun --bun install
RUN bun --bun run build

# Create a smaller image for running the application
FROM oven/bun:1.1

COPY --from=builder /app/build .

EXPOSE 3000

# Start the BUN server
ARG PROD_ORIGIN
ENV ORIGIN=$PROD_ORIGIN
ENV HOST=0.0.0.0
ENV PORT=8080

ENV NODE_ENV=production
ENV PROTOCOL_HEADER="X-Forwarded-Proto"
ENV HOST_HEADER="X-Forwarded-Host"
ENV PORT_HEADER="X-Forwarded-Port"
CMD ["bun", "--bun", "run", "start"]
