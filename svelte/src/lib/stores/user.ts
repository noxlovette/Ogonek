import type { Profile, User } from "$lib/types";
import { writable } from "svelte/store";

export const initialUser: User = {
  id: "",
  username: "",
  name: "",
  role: "student",
  email: "",
};

const initialProfile: Profile = {
  videoCallUrl: "",
  telegramId: "",
  userId: "",
  avatarUrl: "",
};

export const profile = writable<Profile>(initialProfile);
export const user = writable<User>(initialUser);
export function setProfile(data: Profile) {
  profile.update((currentState) => ({
    ...currentState,
    ...data,
  }));
}
export function setUser(data: User) {
  user.update((currentState) => ({
    ...currentState,
    ...data,
  }));
}

export function clearUser() {
  user.update(() => ({
    id: "",
    username: "",
    name: "",
    role: "student",
    email: "",
  }));
}
