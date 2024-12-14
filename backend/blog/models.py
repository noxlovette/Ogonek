from django.db import models
from django.utils.text import slugify
import uuid
from django.core.validators import MinValueValidator, MaxValueValidator

# Create your models here.
class Task(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    description = models.TextField()
    priority = models.PositiveSmallIntegerField(
        default=1,
        validators=[MinValueValidator(1), MaxValueValidator(10)]
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    due_date = models.DateTimeField(null=True, blank=True)
    status = models.CharField(max_length=50, default='pending')

    assignee = models.ForeignKey('users.User', on_delete=models.CASCADE, related_name='tasks')

    def __str__(self):
        return self.title
