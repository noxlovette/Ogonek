from rest_framework.views import APIView
from rest_framework.permissions import IsAuthenticated
from .models import Profile
from .serializers import ProfileSerializer
from rest_framework.response import Response
from django.http import JsonResponse
from django.contrib.auth import authenticate, login
from django.middleware.csrf import get_token
from rest_framework import status
from .permissions import HasAPIKey
from django.utils.decorators import method_decorator

from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
import logging

logger = logging.getLogger(__name__)


class UserDataApi(APIView):
    queryset = Profile.objects.all()
    permission_classes = [IsAuthenticated, HasAPIKey]
    serializer_class = ProfileSerializer

    def get_queryset(self):
        # Filter lessons by the authenticated user
        return self.queryset.filter(assignee=self.request.user)


class LoginAPIView(APIView):
    permission_classes = [HasAPIKey]

    def post(self, request):
        logger.info(f"Headers LOGIN: {request.headers}")
        logger.info(f"Body LOGIN: {request.body}")
        username = request.POST.get("username")
        password = request.POST.get("password")

        user = authenticate(request, username=username, password=password)

        if user is not None:
            login(request, user)
            profile = Profile.objects.get(user=user)
            return Response(
                {
                    "success": True,
                    "message": "Login successful!",
                    "sessionid": request.session.session_key,
                }
            )
        else:
            return JsonResponse(
                {"success": False, "message": "Invalid username or password."},
                status=400,
            )


class CheckSessionAPI(APIView):
    """
    Check if the user is authenticated.
    """

    permission_classes = [HasAPIKey]

    @csrf_exempt
    def get(self, request, *args, **kwargs):
        if request.user.is_authenticated:
            csrf_token = get_token(request)
            profile = Profile.objects.get(user=request.user)
            return Response(
                {
                    "is_authenticated": request.user.is_authenticated,
                    "username": request.user.username,
                    "email": request.user.email,
                    "csrfToken": csrf_token,
                    "quizlet_url": profile.quizlet_url,
                    "client_id": profile.client_id,
                    "first_name": request.user.first_name,
                    "last_name": request.user.last_name,
                }
            )
        else:
            return Response(
                {"is_authenticated": request.user.is_authenticated},
                status=status.HTTP_401_UNAUTHORIZED,
            )
