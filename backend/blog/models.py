from django.db import models
import uuid
from django.core.validators import MinValueValidator, MaxValueValidator
from django.conf import settings
from django.contrib.auth.models import User


class Task(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField()
    priority = models.PositiveSmallIntegerField(
        default=1, validators=[MinValueValidator(1), MaxValueValidator(3)]
    )
    completed = models.BooleanField(default=False)

    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    due_date = models.DateTimeField(null=True, blank=True)

    file = models.FileField(upload_to="tasks/", null=True, blank=True)

    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name="tasks")

    def __str__(self):
        return self.title

    def get_file_url(self):
        if settings.DEBUG:
            base_url = "http://localhost:80"
        else:
            base_url = "https://media.firelight.noxlovette.com"
        return f"{base_url}{self.file.url}"


class Lesson(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField()
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    category = models.CharField(max_length=50, default="grammar")
    topic = models.CharField(max_length=50, default="english")

    manual_date = models.DateTimeField(blank=True, null=True)

    bookmarked = models.BooleanField(default=False)

    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name="lessons")

    def __str__(self):
        return self.title
