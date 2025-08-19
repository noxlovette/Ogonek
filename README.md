# Ogonek

[![CI](https://github.com/noxlovette/ogonek/actions/workflows/ci-build.yml/badge.svg)](https://github.com/noxlovette/ogonek/actions/workflows/ci-build.yml)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white)](https://svelte.dev/)
[![PostgreSQL](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

A high-performance learning management platform designed for scalable educational workflows. Ogonek provides a comprehensive solution for course delivery, student assessment, and spaced repetition learning through a modern web architecture.

## Architecture

### System Overview

Ogonek implements a service-oriented architecture with clear separation between presentation, business logic, and data layers. The monolithic database design ensures consistency while maintaining modular service boundaries.

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   SvelteKit     │    │   Axum Server    │    │   PostgreSQL    │
│   Frontend      │◄──►│   (Rust)         │◄──►│   Database      │
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   Scaleway S3   │
                       │   Object Store  │
                       └─────────────────┘
```

### Backend

**Axum Web Framework**: Built on Tokio's async runtime, providing sub-millisecond response times with efficient resource utilisation.

**Database Layer**: SQLx provides compile-time verified SQL queries with connection pooling and transaction management. Database interactions follow the repository pattern with dedicated CRUD modules.

**File Storage**: Integrated S3-compatible object storage for multimedia content with presigned URL generation for secure, direct client downloads.

**Error Handling**: Centralised error management with structured logging and Sentry integration for production monitoring.

### Frontend

**SvelteKit**: Server-side rendering with progressive enhancement and client-side hydration for optimal performance and SEO.

**Type Safety**: Full TypeScript integration.

**State Management**: Svelte stores with reactive data flow patterns.

**Internationalisation**: Paraglide-based translation system with compile-time optimisation.

## Core Features

### Educational Content Management

**Lesson Orchestration**: Hierarchical content organisation with multimedia support and progress tracking. Teachers can create structured learning paths with embedded assessments and resource attachments.

**Task Assignment System**: Task distribution with deadline management and submission tracking. Supports file attachments up to 100MB with real-time upload progress.

**Spaced Repetition Engine**: Intelligent flashcard scheduling based on forgetting curve algorithms. Adaptive difficulty adjustment based on user performance metrics.

### Content Delivery

**Responsive Design**: Mobile-first interface with adaptive layouts for tablets and desktop environments.

**Updates**: Telegram and iOS notifications for assignment updates, new content, and system announcements.

## Authentication & Authorisation

### Security Architecture

**JWT-based Authentication**: Stateless token system with configurable expiration and refresh token rotation. Tokens include user roles and permissions for fine-grained access control.

**Role-based Access Control (RBAC)**:
- **Teachers**: Full access to course creation, student management, and analytics
- **Students**: Access to assigned content, progress tracking, and submission capabilities

### Security Features

**Password Security**: Argon2 hashing with configurable work factors and salt generation.

**Session Management**: Token storage in cookies.

**API Security**: CORS configuration, request validation middleware.

**Data Protection**: Environment-based secrets management.

## Deployment

Containerised deployment using Docker Compose with multi-stage builds for production optimisation. Environment-based configuration with automated database migrations and integrated health monitoring.

---

For technical support and deployment assistance: [danila.volkov@noxlovette.com](mailto:danila.volkov@noxlovette.com)
