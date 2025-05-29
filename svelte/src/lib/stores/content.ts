import type { LessonSmall, Student } from "$lib/types";
import { writable } from "svelte/store";

const createLessonStore = () => {
  const { subscribe, set, update } = writable<LessonSmall[]>([]);

  return {
    subscribe,
    setLessons: (lessons: LessonSmall[]) => set(lessons),
    addLesson: (lesson: LessonSmall) =>
      update((lessons) => [...lessons, lesson]),
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
export const lessonStore = createLessonStore();
export const studentStore = createStudentStore();
