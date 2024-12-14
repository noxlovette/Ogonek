from django.contrib import admin
from .models import Task, Recommendation, Comment, User

# Register your models here.
admin.site.register(Task)
admin.site.register(Recommendation)
admin.site.register(Comment)
admin.site.register(User)