from rest_framework import viewsets
from .models import Task, User
from .serializers import TaskSerializer, UserSerializer
from rest_framework.views import APIView
from django.http import JsonResponse
from django.contrib.auth import authenticate, login

class TaskViewSet(viewsets.ModelViewSet):
    queryset = Task.objects.all()
    serializer_class = TaskSerializer
 
class UserDataApi(APIView):
    queryset = User.objects.all()
    serializer_class = UserSerializer

class LoginAPIView(APIView):
    def post(self, request):
        username = request.POST.get('username')
        password = request.POST.get('password')

        user = authenticate(request, username=username, password=password)

        if user is not None:
            login(request, user)
            return JsonResponse({'success': True, 'message': 'Login successful!'})
        else:
            return JsonResponse({'success': False, 'message': 'Invalid username or password.'}, status=400)