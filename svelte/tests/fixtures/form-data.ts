import { testUsers, validPasswords } from "./users";

export const formInputs = {
  validTeacher: {
    username: testUsers.teacher.username,
    password: validPasswords.teacher,
  },

  validStudent: {
    username: testUsers.student.username,
    password: validPasswords.student,
  },

  invalidCredentials: {
    username: "invalid@test.com",
    password: "wrongpassword",
  },

  emptyFields: {
    username: "",
    password: "",
  },

  onlyUsername: {
    username: testUsers.teacher.username,
    password: "",
  },

  onlyPassword: {
    username: "",
    password: validPasswords.teacher,
  },
} as const;
