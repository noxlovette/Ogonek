from django.db import models
from django.utils.text import slugify
import uuid
from django.core.validators import MinValueValidator, MaxValueValidator
from django.contrib.auth.models import AbstractUser
from django.contrib.auth.hashers import make_password
from django.conf import settings

class User(AbstractUser):
    quizlet_url = models.URLField(null=True, blank=True)
    def save(self, *args, **kwargs):
        if self.has_usable_password:
            self.password = make_password(self.password)
        super().save(*args, **kwargs)

    

# Create your models here.
class Task(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField()
    priority = models.PositiveSmallIntegerField(
        default=1,
        validators=[MinValueValidator(1), MaxValueValidator(10)]
    )
    completed = models.BooleanField(default=False)

    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    due_date = models.DateTimeField(null=True, blank=True)

    file = models.FileField(upload_to='tasks/', null=True, blank=True)


    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name='tasks')

    def __str__(self):
        return self.title
    
    def get_file_url(self):
        if settings.DEBUG:
            base_url = 'http://localhost:80'
        else:
            base_url = 'https://media.firelight.noxlovette.com'
        return f"{base_url}{self.file.url}"
    
class Recommendation(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField()
    priority = models.PositiveSmallIntegerField(
        default=1,
        validators=[MinValueValidator(1), MaxValueValidator(10)]
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    due_date = models.DateTimeField(null=True, blank=True)

    completed = models.BooleanField(default=False)
    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name='recommendations')

    def __str__(self):
        return self.title
    
class Comment(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    content = models.TextField()
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    category = models.CharField(max_length=50, default='grammmar')

    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name='comments')

    def __str__(self):
        return self.content


class Lesson(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField()
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    category = models.CharField(max_length=50, default='grammar')
    topic = models.CharField(max_length=50, default='english')

    manual_date = models.DateTimeField(blank=True, null=True)

    bookmarked = models.BooleanField(default=False)

    assignee = models.ForeignKey(User, on_delete=models.CASCADE, related_name='lessons')

    def __str__(self):
        return self.title