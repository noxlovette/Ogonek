FROM node:24-alpine AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install
COPY . .
RUN pnpm build

FROM node:24-alpine
WORKDIR /app
COPY --from=builder /app/build .
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --prod && \
    rm -rf /root/.pnpm-store
CMD ["node", "index.js"]