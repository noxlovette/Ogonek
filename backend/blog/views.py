from rest_framework import viewsets
from .models import Task, User
from .serializers import TaskSerializer, UserSerializer
from rest_framework.views import APIView

class TaskViewSet(viewsets.ModelViewSet):
    queryset = Task.objects.all()
    serializer_class = TaskSerializer
 
class UserDataApi(APIView):
    queryset = User.objects.all()
    serializer_class = UserSerializer