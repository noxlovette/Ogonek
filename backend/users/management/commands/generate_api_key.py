from django.core.management.base import BaseCommand
import secrets
import base64


class Command(BaseCommand):
    help = "Generate UUIDs for existing formulas"

    def handle(self, *args, **kwargs):
        api_key = (
            base64.urlsafe_b64encode(secrets.token_bytes(32))
            .decode("utf-8")
            .rstrip("=")
        )
        self.stdout.write(self.style.SUCCESS(f"Generated {api_key}"))
