from django.contrib import admin
from .models import Task, Recommendation, Comment, User, Lesson

# Register your models here.
admin.site.register(Task)
admin.site.register(Recommendation)
admin.site.register(Comment)
admin.site.register(User)

# Use decorator for Lessons model with custom admin settings
@admin.register(Lesson)
class LessonsAdmin(admin.ModelAdmin):
    list_display = ('title', 'assignee', 'created_at')
    list_filter = ('assignee', 'created_at')
    search_fields = ('title', 'content')
    date_hierarchy = 'created_at'
    ordering = ('-created_at',)