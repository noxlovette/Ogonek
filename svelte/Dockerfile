FROM node:23-alpine AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install
COPY . .
RUN pnpm build

FROM node:23-alpine
WORKDIR /app
COPY --from=builder /app/build ./build
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --prod && \
    rm -rf /root/.pnpm-store
CMD ["node", "build/index.js"]