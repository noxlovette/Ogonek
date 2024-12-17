from django.db import models
from django.contrib.auth.models import User
import uuid

class Profile(models.Model):
    client_id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    user = models.OneToOneField(User, on_delete=models.CASCADE, related_name='profile')
    quizlet_url = models.URLField(null=True, blank=True)

    def save(self, *args, **kwargs):
        if not self.client_id:
            self.client_id = uuid.uuid4()
        return super().save(*args, **kwargs)
    
    def __str__(self):
        return self.user.username