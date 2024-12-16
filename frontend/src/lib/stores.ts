import { writable } from "svelte/store";

export  const user = writable({
    is_authenticated: false,
    username: "",
    email: "",
    quizlet_url: "",
  });

  export function setUser(data) {
    user.update((currentState) => ({
      ...currentState,
      ...data,
    }));
  }
  
  export function clearUser() {
    user.update(() => ({
      is_authenticated: false,
      username: "",
      email: "",
      quizlet_url: "",
        
    }));
  }
  
  
  export const notification = writable({
    message: "",
    type: "none",
  });


  export const lesson = writable({
    title: "",
    description: "",
    bookmarked: false,
    manual_date: "",
    content: "",
    created_at: "",
    updated_at: "",
    topic: "",
  });
