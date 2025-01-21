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
# this variable can either be hardcoded here or set up in an .env file in svelte's directory
#ENV ORIGIN="http://localhost:3333"
CMD ["node", "build/index.js"]