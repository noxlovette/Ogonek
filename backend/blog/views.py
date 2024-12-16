from rest_framework import viewsets
from .models import Task, User, Lesson
from .serializers import TaskSerializer, UserSerializer, LessonSerializer
from rest_framework.views import APIView
from django.http import JsonResponse
from rest_framework.response import Response
from django.contrib.auth import authenticate, login
from django.middleware.csrf import get_token
from rest_framework import permissions, status
from rest_framework.permissions import IsAuthenticated
from django_filters.rest_framework import DjangoFilterBackend
from django.http import FileResponse
from django.views import View
from django.core.cache import cache

class TaskViewSet(viewsets.ModelViewSet):
    queryset = Task.objects.all()
    permission_classes = [IsAuthenticated]
    serializer_class = TaskSerializer

    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)
    
class DownloadFileView(View):
    permission_classes = [IsAuthenticated]

    def get(self, request, file_id):    
        file_instance = Task.objects.get(id=file_id)  # Fetch the correct model instance
        file_path = file_instance.file.path  # Get the file path
        response = FileResponse(open(file_path, 'rb'))
        response['Content-Disposition'] = f'attachment; filename="{file_instance.file.name}"'
        return response
 
class UserDataApi(APIView):
    queryset = User.objects.all()
    permission_classes = [IsAuthenticated]
    serializer_class = UserSerializer

    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)


class LessonViewSet(viewsets.ModelViewSet):
    queryset = Lesson.objects.all()
    serializer_class = LessonSerializer
    permission_classes = [IsAuthenticated]
    filter_backends = [DjangoFilterBackend]
    filterset_fields = ['assignee']  # Assuming there's a field 'assignee' in Lesson model for the user

    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)

class LoginAPIView(APIView):
    def post(self, request):
        username = request.POST.get('username')
        password = request.POST.get('password')

        user = authenticate(request, username=username, password=password)

        if user is not None:
            login(request, user)
            return Response(
                {
                    "success": True,
                    "message": "Login successful!",
                    "username": user.username,
                    "is_authenticated": user.is_authenticated,
                    "email": user.email,
                    "sessionid": request.session.session_key,
                    "quizlet_url": user.quizlet_url,
                }
            )
        else:
            return JsonResponse({'success': False, 'message': 'Invalid username or password.'}, status=400)
        


class CheckSessionAPI(APIView):
    """
    Check if the user is authenticated.
    """

    def get(self, request, *args, **kwargs):
        if request.user.is_authenticated:
            csrf_token = get_token(request)
            return Response(
                {
                    "is_authenticated": request.user.is_authenticated,
                    "quizlet_url": request.user.quizlet_url,
                    "username": request.user.username,
                    "email": request.user.email,
                    "csrfToken": csrf_token,
                }
            )
        else:
            return Response(
                {"is_authenticated": request.user.is_authenticated},
                status=status.HTTP_401_UNAUTHORIZED,
            )