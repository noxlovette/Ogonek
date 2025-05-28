export const testUsers = {
  teacher: {
    id: 1,
    username: "teacher@test.com",
    email: "teacher@test.com",
    role: "teacher",
    firstName: "John",
    lastName: "Teacher",
  },
  student: {
    id: 2,
    username: "student@test.com",
    email: "student@test.com",
    role: "student",
    firstName: "Jane",
    lastName: "Student",
  },
  invalidUser: {
    username: "invalid@test.com",
    password: "wrongpassword",
  },
} as const;

export const validPasswords = {
  teacher: "teacher123",
  student: "student123",
} as const;
