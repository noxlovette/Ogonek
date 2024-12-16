from django.contrib import admin
from .models import Task, Recommendation, Comment, User, Lesson
from django.utils.html import format_html
from django.urls import reverse


# Register your models here.

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


class TaskAdmin(admin.ModelAdmin):
    # Fields to display in the list view
    list_display = ('assignee', 'title', 'priority', 'completed', 'due_date', 'created_at', 'updated_at')
    
    # Fields to filter the list view by
    list_filter = ('completed', 'priority', 'created_at', 'updated_at', 'assignee')
    
    # Fields to search in
    search_fields = ['title', 'content']
    
    # Fields that can be edited directly from the list view
    list_editable = ('priority', 'completed')
    
    # Fields to include in the form for adding/changing entries
    fieldsets = (
        (None, {
            'fields': ('title', 'content', 'priority', 'completed', 'due_date', 'assignee', 'file')
        }),
        ('Date Information', {
            'fields': ('created_at', 'updated_at'),
            'classes': ('collapse',)  # This will make the fieldset collapsible
        }),
    )
    
    # Make 'created_at' and 'updated_at' read-only
    readonly_fields = ('created_at', 'updated_at')

# Register the Task model with the custom TaskAdmin
admin.site.register(Task, TaskAdmin)