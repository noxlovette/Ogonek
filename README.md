# Ogonek

EduCards is a modern, full-stack web application designed to facilitate teaching and learning through flashcards, task management, and lesson planning. The platform connects teachers and students in a collaborative learning environment.

## Overview

This application provides a comprehensive learning management system with the following core features:

- **Flashcard Learning System**: Create, share, and study decks of flashcards with spaced repetition algorithms (SM2)
- **User Management**: Teacher and student roles with appropriate permissions and profiles
- **Lesson Planning**: Create and assign lessons to students
- **Task Management**: Assign and track tasks with priorities and due dates
- **File Management**: Upload, organize, and share files with S3 storage integration (next release)

## Tech Stack

### Backend

- **Rust** with **Axum** framework for a high-performance API
- **SQLx** for type-safe database interactions with PostgreSQL
- **JWT** authentication with role-based access control
- **S3 Integration** for file storage (Scaleway)

### Frontend

- **Svelte** with component-based architecture
- **TailwindCSS** for styling
- **TypeScript** for type safety

### Infrastructure

- **Docker Compose** for containerization and deployment
- **PostgreSQL** for relational data storage
- **Redis** for caching

## Core Features

### Spaced Repetition Learning

The application implements the SM2 algorithm for optimized flashcard learning, automatically scheduling reviews based on user performance.

### Teacher-Student Relationship

- Teachers can create content, assign tasks and lessons
- Students can access assigned content, track their learning progress, study and create flashcards
- Both roles have personalized dashboards

### Content Management

- Create and organize flashcard decks
- Author lesson content with markdown support
- Assign and manage tasks with priority levels and due dates

### File System (next release)

- Upload and organize files in folders
- Attach files to tasks and lessons
- Secure file storage using S3-compatible storage

## API Structure

The API is organized around these primary resources:

- `/auth` - Authentication endpoints
- `/user` - User profile management
- `/deck` - Flashcard deck management
- `/learning` - Spaced repetition learning endpoints
- `/lesson` - Lesson content management
- `/task` - Task management
- `/student` - Teacher-student relationship management
- `/profile` - User profile customization
- `/file` - File management system

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Scaleway S3 compatible storage account (or alternative S3 provider)

### Environment Variables

See the .env.example in the axum/ and svelte/ directories

### Installation

1. Clone the repository
2. Set up environment variables
3. Run with Docker Compose:
   ```
   docker compose -f compose.dev.yaml up -d
   ```

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).

This means:

- You can use, modify, and distribute this software
- If you modify and distribute it, you must make your modifications available under the same license
- If you run a modified version of the software on a server that users interact with (even over a network), you must make the source code available to those users

This ensures that all improvements to the code remain open source, while allowing the original creator to maintain certain commercial advantages.

For more information, see the [full license text](https://www.gnu.org/licenses/agpl-3.0.en.html).
