import os
from django.core.management.base import BaseCommand
from django.contrib.auth.hashers import make_password
from blog.models import User, Lesson

class Command(BaseCommand):
    help = 'Parse Markdown files, populate User and Lesson models, and hash user passwords'

    def handle(self, *args, **options):
        base_dir = os.path.dirname(os.path.abspath(__file__))
        folder_path = os.path.join(base_dir, 'students_data')  # Assuming students_data is in the same dir as the script

        for student_folder in os.listdir(folder_path):
            student_folder_path = os.path.join(folder_path, student_folder)
            if os.path.isdir(student_folder_path):  # Check if it's a directory
                for filename in os.listdir(student_folder_path):
                    if filename.endswith('.md'):
                        file_path = os.path.join(student_folder_path, filename)
                        
                        # Extract the first word from the filename as the username
                        username = filename.split(' ', 1)[0].split('.')[0]
                        
                        # Check if user exists, if not create one with hashed password
                        user, created = User.objects.get_or_create(
                            username=username
                        )
                        
                        # If this is a new user or we want to rehash for existing users
                        if created or not user.has_usable_password():  # Check if password is already hashed
                            user.password = make_password(username)  # Hash the password
                            user.save()

                        # Read the content of the file
                        try:
                            with open(file_path, 'r', encoding='utf-8') as file:
                                content = file.read()
                        except FileNotFoundError:
                            self.stdout.write(self.style.ERROR(f"File not found: {file_path}"))
                            continue  # Skip this file if not found
                        
                        # Create Lesson instance
                        Lesson.objects.create(
                            assignee=user,
                            title=filename,
                            content=content
                        )

        self.stdout.write(self.style.SUCCESS('Successfully processed Markdown files, created Users and Lessons, and hashed passwords.'))