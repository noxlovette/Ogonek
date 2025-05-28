export const loginResponses = {
  teacherSuccess: {
    redirect: "/t/dashboard",
  },

  studentSuccess: {
    redirect: "/s/dashboard",
  },

  invalidCredentials: {
    error: "Invalid username or password",
  },

  serverError: {
    error: "Something went wrong",
  },
} as const;
