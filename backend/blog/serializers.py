from rest_framework import serializers
from .models import Article, Category, Tag, Concept

class ArticleSerializer(serializers.ModelSerializer):
    tags = serializers.ListField(child=serializers.CharField(), write_only=True)
    tags_display = serializers.SerializerMethodField()

    class Meta:
        model = Article
        fields = '__all__'

    def get_tags_display(self, obj):
        return [tag.name for tag in obj.tags.all()]

    def create(self, validated_data):
        tags_data = validated_data.pop('tags', [])
        article = Article.objects.create(**validated_data)
        self._update_tags(article, tags_data)
        return article

    def update(self, instance, validated_data):
        tags_data = validated_data.pop('tags', None)
        
        # Update article fields
        for attr, value in validated_data.items():
            setattr(instance, attr, value)
        instance.save()

        # If tags data is provided, associate or create the tags
        if tags_data is not None:
            self._update_tags(instance, tags_data)
        
        return instance

    def _update_tags(self, article, tags_data):
        # Clear current tags
        article.tags.clear()

        # Add tags based on names provided
        for tag_name in tags_data:
            tag_name = tag_name.strip().lower()  # Clean up the name
            tag, created = Tag.objects.get_or_create(name=tag_name)  # Lookup or create the tag
            article.tags.add(tag)  # Associate the tag with the article


class CategorySerializer(serializers.ModelSerializer):
    class Meta:
        model = Category
        fields = '__all__'
        

class TagSerializer(serializers.ModelSerializer):
    class Meta:
        model = Tag
        fields = '__all__'

class ConceptSerializer(serializers.ModelSerializer):
    class Meta:
        model = Concept
        fields = '__all__'