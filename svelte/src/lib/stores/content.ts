import type { Student } from "$lib/types";
import { writable } from "svelte/store";

const createStudentStore = () => {
  const { subscribe, set, update } = writable<Student[]>([]);

  return {
    subscribe,
    setStudents: (students: Student[]) => set(students),
    addStudent: (student: Student) =>
      update((students) => [...students, student]),
    reset: () => set([]),
  };
};
export const studentStore = createStudentStore();
