import z from "zod";

export const signinBodyPassMin = 8;

export const signinBodyPassMax = 32;
export const signinBodyUsernameMin = 3;

export const signinBodyUsernameMax = 50;

export const signinBody = z.object({
  pass: z.string().min(signinBodyPassMin).max(signinBodyPassMax),
  username: z.string().min(signinBodyUsernameMin).max(signinBodyUsernameMax),
});

export const signupBodyNameMin = 3;

export const signupBodyNameMax = 100;
export const signupBodyPassMin = 8;

export const signupBodyPassMax = 128;
export const signupBodyUsernameMin = 2;

export const signupBodyUsernameMax = 50;

export const signupBody = z.object({
  email: z.email(),
  name: z.string().min(signupBodyNameMin).max(signupBodyNameMax),
  pass: z.string().min(signupBodyPassMin).max(signupBodyPassMax),
  role: z.string(),
  username: z.string().min(signupBodyUsernameMin).max(signupBodyUsernameMax),
});
