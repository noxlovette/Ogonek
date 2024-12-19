from django.core.management.base import BaseCommand
from blog.models import User
import uuid


class Command(BaseCommand):
    help = "Generate client_id (UUID4) for all users without one"

    def handle(self, *args, **options):
        users = User.objects.all()
        updated_count = 0

        for user in users:
            user.client_id = uuid.uuid4()
            user.save()
            updated_count += 1
            self.stdout.write(
                self.style.SUCCESS(f"UUID generated for user: {user.username}")
            )

        self.stdout.write(
            self.style.SUCCESS(f"UUIDs generated for {updated_count} users")
        )
