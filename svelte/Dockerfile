# Build stage
FROM node:20-slim AS builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

# Copy package files
COPY pnpm-lock.yaml package.json ./

# Install curl for healthchecks
RUN apt-get update && apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

# Install deps with cache mount
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile --prod

# Copy source
COPY package.json pnpm-lock.yaml ./
COPY tsconfig*.json ./ 
COPY src/ src/

# Build the app (assuming you have a build script)
RUN pnpm run build

# Production stage
FROM node:20-slim
ENV NODE_ENV=production
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

# Copy only production dependencies
COPY --from=builder /app/package.json /app/pnpm-lock.yaml ./
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile --prod

# Copy built app
COPY --from=builder /app/dist ./dist

# Create non-root user
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nodejs
USER nodejs

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:3000/health || exit 1

# Use production command
CMD ["pnpm", "start"]