from .base import *
import os

DEBUG = False
SESSION_SAVE_EVERY_REQUEST = True
ALLOWED_HOSTS = ['admin.noxlovette.com', 'backend-firelight-prod', 'localhost', 'backend', 'media.noxlovette.com']

# SECURE_PROXY_SSL_HEADER = ('HTTP_X_FORWARDED_PROTO', 'https')
# USE_X_FORWARDED_HOST = True
# FORCE_SCRIPT_NAME = ''
STATIC_URL = 'https://media.noxlovette.com/static/'
MEDIA_URL = 'https://media.noxlovette.com/media/'

CORS_ALLOWED_ORIGINS = [
    "https://firelight.noxlovette.com",
    "http://frontend:3000",
    "http://localhost:3000",
    "https://admin.noxlovette.com",
    "https://media.noxlovette.com",
]
ROOT_URLCONF = "backend.urls"
DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.postgresql",
        "NAME": os.environ.get("PGNAME", "firelight-production"),
        "USER": os.environ.get("PGUSER", "postgres"),
        "PASSWORD": os.environ.get("PGPASSWORD", "changeme"),
        "HOST": os.environ.get("PGHOST", "db"),
        "PORT": os.environ.get("PGPORT", "5432"),
    }
}

REST_FRAMEWORK = {
    "DEFAULT_THROTTLE_CLASSES": [
        "rest_framework.throttling.AnonRateThrottle",
        "rest_framework.throttling.UserRateThrottle",
    ],
    "DEFAULT_THROTTLE_RATES": {"anon": "100/day", "user": "1000/day"},
}

# Email configuration for production - Use actual SMTP server or a service like SendGrid
# EMAIL_BACKEND = 'django.core.mail.backends.smtp.EmailBackend'
# EMAIL_HOST = 'smtp.yourprovider.com'
# EMAIL_PORT = 587
# EMAIL_USE_TLS = True
# EMAIL_HOST_USER = 'your-account@yourprovider.com'
# EMAIL_HOST_PASSWORD = os.environ.get('EMAIL_HOST_PASSWORD')

# Logging - Ensure logs are written to a location accessible by your deployment environment
# LOGGING['handlers']['info_file']['filename'] = '/path/to/production/logs/info.log'
# LOGGING['handlers']['error_file']['filename'] = '/path/to/production/logs/error.log'
# LOGGING['handlers']['db']['filename'] = '/path/to/production/logs/db.log'

if "debug_toolbar" in INSTALLED_APPS:
    INSTALLED_APPS.remove("debug_toolbar")
if "debug_toolbar.middleware.DebugToolbarMiddleware" in MIDDLEWARE:
    MIDDLEWARE.remove("debug_toolbar.middleware.DebugToolbarMiddleware")

# Other production-specific settings might include:
# - Rate limiting
# - Security headers
# - CORS settings if needed
# - Session settings for longer-term or more secure configurations
