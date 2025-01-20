# What is this?
This is the compose that pulls tested images from the github registry and deploys the application on the server. Is beneficial because in case of failure all it would take is roll back to the previous version and pull the images that worked. This makes the whole setup maintable.

## Environment
### Axum
Axum expects an API_KEY that should be identica to the one on Svelte, DATABASE_URL for sqlx, and two pairs of rsa256 keys for the jwt generation (JWT_PRIVATE_KEY, JWT_PUBLIC_KEY, JWT_REFRESH_PRIVATE_KEY, JWT_REFRESH_PUBLIC_KEY).

### Svelte
Svelte needs an ORIGIN to handle csrf (although you could locally just go to vite.config.js and turn csrf off). It needs the PUBLIC_spki for the accessToken, PUBLIC_alg which is the algo for the key, the UPLOAD_DIR, the API_KEY_AXUM to handle communication with the backend, and the TELEGRAM_API for the bot.

### Database
POSTGRES_DB, POSTGRES_USER, POSTGRES_PASSWORD. You will use these to construct the DATABASE_URL for your axum anyway.