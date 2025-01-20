# Stage 1: Build the application
FROM node:23-slim AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install
COPY . .
RUN pnpm build

# Stage 2: Production image
FROM node:23-slim
WORKDIR /app
COPY --from=builder /app/build ./build
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile --prod
CMD ["node", "build/index.js"]
