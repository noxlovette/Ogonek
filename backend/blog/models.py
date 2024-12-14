from django.db import models
from django.utils.text import slugify
import uuid

# Create your models here.
class Article(models.Model):
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    title = models.CharField(max_length=255)
    content = models.TextField(help_text="Markdown and LaTeX raw content", blank=True)
    processed_html = models.TextField(help_text="HTML content processed from markdown and LaTeX", blank=True)

    category= models.ForeignKey('Category', on_delete=models.SET_NULL, null=True)
    tags = models.ManyToManyField('Tag', blank=True, null=True, related_name='articles')
    concepts = models.ManyToManyField('Concept', blank=True, related_name='articles')

    related_articles = models.ManyToManyField('self', blank=True, symmetrical=True)

    slug = models.SlugField(max_length=255, unique=True, help_text="URL friendly title", blank=True)

    difficulty = models.IntegerField(default=0, help_text="Difficulty level from 0 to 10")

    published = models.BooleanField(default=False)

    def save(self, *args, **kwargs):
        self.slug = slugify(self.title)
        super().save(*args, **kwargs)

    def __str__(self):
        return self.title
    
class Category(models.Model):
    # A large concept, such as organic chemistry
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    name = models.CharField(max_length=255)
    description = models.TextField()

    def __str__(self):
        return self.name
    
class Tag(models.Model):
    # a very small concept, such as benzol
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    name = models.CharField(max_length=255)

    def __str__(self):
        return self.name

class Concept(models.Model):
    # Electronegativity, atomic structure
    id = models.UUIDField(primary_key=True, editable=False, default=uuid.uuid4)
    name = models.CharField(max_length=255)

    def __str__(self):
        return self.name