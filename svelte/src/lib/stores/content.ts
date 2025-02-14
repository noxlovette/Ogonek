import type { Lesson, Student, Task } from "$lib/types";
import { writable } from "svelte/store";

const createLessonStore = () => {
  const { subscribe, set, update } = writable<Lesson[]>([]);

  return {
    subscribe,
    setLessons: (lessons: Lesson[]) => set(lessons),
    addLesson: (lesson: Lesson) => update((lessons) => [...lessons, lesson]),
    reset: () => set([]),
  };
};

const createTaskStore = () => {
  const { subscribe, set, update } = writable<Task[]>([]);

  return {
    subscribe,
    setTasks: (tasks: Task[]) => set(tasks),
    addTask: (task: Task) => update((tasks) => [...tasks, task]),
    reset: () => set([]),
  };
};

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
export const taskStore = createTaskStore();
export const lessonStore = createLessonStore();
export const studentStore = createStudentStore();

export const isSearchOpen = writable(false);

export const tableQuery = writable("");
