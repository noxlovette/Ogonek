from rest_framework.permissions import BasePermission
import os

API_KEY = os.getenv("API_KEY_DJANGO")


class HasAPIKey(BasePermission):
    def has_permission(self, request, view):
     
        provided_key = request.headers.get("X-API-Key")
        return provided_key == API_KEY
