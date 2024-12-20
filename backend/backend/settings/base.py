from pathlib import Path
import os

# Build paths inside the project like this: BASE_DIR / 'subdir'.
BASE_DIR = (
    Path(__file__).resolve().parent.parent.parent
)  # Adjusted for nested settings folder

SECRET_KEY = os.environ.get("DJANGO_SECRET_KEY")

ROOT_URLCONF = "backend.urls"

# Application definition
INSTALLED_APPS = [
    "django.contrib.admin",
    "django.contrib.auth",
    "django.contrib.contenttypes",
    # 'corsheaders',
    "debug_toolbar",
    "django.contrib.sessions",
    "django.contrib.messages",
    "django.contrib.staticfiles",
    "blog",
    "users",
]

STATIC_ROOT = os.path.join(BASE_DIR, "static")
# Common settings like MIDDLEWARE, TEMPLATES, etc.

LOGGING = {
    "version": 1,
    "disable_existing_loggers": False,
    "formatters": {
        "standard": {"format": "%(asctime)s [%(levelname)s] %(name)s: %(message)s"},
    },
    "handlers": {
        "info_file": {
            "level": "INFO",
            "class": "logging.FileHandler",
            "formatter": "standard",
            "filename": os.path.join(BASE_DIR, "logs", "info.log"),
        },
        "error_file": {
            "level": "ERROR",
            "class": "logging.FileHandler",
            "formatter": "standard",
            "filename": os.path.join(BASE_DIR, "logs", "error.log"),
        },
        "db": {
            "level": "DEBUG",
            "class": "logging.FileHandler",
            "formatter": "standard",
            "filename": os.path.join(BASE_DIR, "logs", "db.log"),
        },
    },
    "loggers": {
        "django": {
            "handlers": ["info_file", "error_file"],
            "level": "INFO",
            "propagate": True,
        },
        "django.db.backends": {
            "level": "DEBUG",
            "handlers": ["db"],
        },
    },
}

CORS_ALLOW_CREDENTIALS = True
CORS_ALLOW_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"]
CSRF_TRUSTED_ORIGINS = [
    "https://next-precisely-piranha.ngrok-free.app",
    "http://frontend:3000",
    "https://*.noxlovette.com",
    "http://localhost:3000",
    "https://admin.noxlovette.com",
    "http://localhost:8080",
    "http://localhost:8000",
]
CSRF_COOKIE_HTTPONLY = True
CSRF_COOKIE_SECURE = True
SESSION_COOKIE_SECURE = True
SESSION_COOKIE_SAMESITE = "None"
CSRF_COOKIE_SAMESITE = "None"
# CSRF_COOKIE_DOMAIN = "test.bcfapp.app"
SESSION_COOKIE_SECURE = True

SECRET_KEY = os.environ.get("DJANGO_SECRET_KEY")

SESSION_COOKIE_AGE = 60 * 60 * 24  # 1 day in seconds

# Ensure sessions do not expire when the browser closes
SESSION_EXPIRE_AT_BROWSER_CLOSE = False

SESSION_ENGINE = "django.contrib.sessions.backends.db"
# SESSION_CACHE_ALIAS = "default"

MIDDLEWARE = [
    #  'corsheaders.middleware.CorsMiddleware',
    "django.middleware.security.SecurityMiddleware",
    "django.contrib.sessions.middleware.SessionMiddleware",
    "debug_toolbar.middleware.DebugToolbarMiddleware",
    "django.middleware.common.CommonMiddleware",
    "django.middleware.csrf.CsrfViewMiddleware",
    "django.contrib.auth.middleware.AuthenticationMiddleware",
    "django.contrib.messages.middleware.MessageMiddleware",
    "django.middleware.clickjacking.XFrameOptionsMiddleware",
]


# CACHES = {
#     "default": {
#         "BACKEND": "django_redis.cache.RedisCache",
#         "LOCATION": "redis://redis:6379/2",  # Change to your Redis server's location
#         "OPTIONS": {
#             "CLIENT_CLASS": "django_redis.client.DefaultClient",
#         }
#     }
# }




TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": [],
        "APP_DIRS": True,
        "OPTIONS": {
            "context_processors": [
                "django.template.context_processors.debug",
                "django.template.context_processors.request",
                "django.contrib.auth.context_processors.auth",
                "django.contrib.messages.context_processors.messages",
            ],
        },
    },
]
USE_TZ = True
TIME_ZONE = "Europe/Berlin"
WSGI_APPLICATION = "backend.wsgi.application"

DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.postgresql",
        "NAME": os.environ.get("PGNAME"),
        "USER": os.environ.get("PGUSER"),
        "PASSWORD": os.environ.get("PGPASSWORD"),
        "HOST": os.environ.get("PGHOST"),
        "PORT": os.environ.get("PGPORT"),
    }
}

PASSWORD_HASHERS = [
    "django.contrib.auth.hashers.PBKDF2PasswordHasher",
    "django.contrib.auth.hashers.PBKDF2SHA1PasswordHasher",
    # Other hashers...
]

AUTH_PASSWORD_VALIDATORS = [
    {
        "NAME": "django.contrib.auth.password_validation.UserAttributeSimilarityValidator",
    },
    {
        "NAME": "django.contrib.auth.password_validation.MinimumLengthValidator",
    },
    {
        "NAME": "django.contrib.auth.password_validation.CommonPasswordValidator",
    },
    {
        "NAME": "django.contrib.auth.password_validation.NumericPasswordValidator",
    },
]

LANGUAGE_CODE = "en-gb"

TIME_ZONE = "UTC"

USE_I18N = True

USE_TZ = True

STATIC_URL = "static/"

DEFAULT_AUTO_FIELD = "django.db.models.BigAutoField"

#this is relavant where django saves files
MEDIA_ROOT = os.path.join(BASE_DIR, "media")
MEDIA_URL = "http://localhost:8080/media/"
