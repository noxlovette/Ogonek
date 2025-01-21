# Stage 1: Build the application
FROM node:23-slim AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install
COPY . .
RUN pnpm build

# Stage 2: Production image
FROM node:23-slim

# Create a non-root user
RUN groupadd -r nodejs && useradd -r -g nodejs nodejs

WORKDIR /app

# Create and set permissions for uploads directory
RUN mkdir -p /app/static/uploads && \
    chown -R nodejs:nodejs /app/static/uploads

COPY --from=builder /app/build ./build
COPY package.json pnpm-lock.yaml ./

RUN corepack enable && \
    pnpm install --frozen-lockfile --prod && \
    chown -R nodejs:nodejs /app

# Switch to non-root user
USER nodejs

CMD ["node", "build/index.js"]