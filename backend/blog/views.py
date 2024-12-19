from rest_framework import viewsets
from .models import Task, Lesson
from .serializers import TaskSerializer, LessonSerializer
from django.http import JsonResponse
from rest_framework.permissions import IsAuthenticated
from users.permissions import HasAPIKey
from django_filters.rest_framework import DjangoFilterBackend
from django.http import FileResponse
from django.views import View
from django.views.decorators.csrf import csrf_exempt



class TaskViewSet(viewsets.ModelViewSet):
    queryset = Task.objects.all()
    permission_classes = [IsAuthenticated, HasAPIKey]
    serializer_class = TaskSerializer
    @csrf_exempt
    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)



class DownloadFileView(View):
    permission_classes = [IsAuthenticated, HasAPIKey]
    @csrf_exempt
    def get(self, request, file_id):
        file_instance = Task.objects.get(id=file_id)  # Fetch the correct model instance
        file_path = file_instance.file.path  # Get the file path
        response = FileResponse(open(file_path, "rb"))
        response["Content-Disposition"] = (
            f'attachment; filename="{file_instance.file.name}"'
        )
        return response



class LessonViewSet(viewsets.ModelViewSet):
    queryset = Lesson.objects.all()
    serializer_class = LessonSerializer
    permission_classes = [IsAuthenticated, HasAPIKey]
    filter_backends = [DjangoFilterBackend]
    filterset_fields = [
        "assignee"
    ]  # Assuming there's a field 'assignee' in Lesson model for the user
    @csrf_exempt
    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)
