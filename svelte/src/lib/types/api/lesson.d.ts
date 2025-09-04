import type { components } from "./gen/openapi";
/**
 * The lesson type used in lists and cards
 */
export type LessonSmall = components["schemas"]["LessonSmall"];

/**
 * The body that updates a lesson
 */
export type LessonUpdateBody = components["schemas"]["LessonUpdate"];

/**
 * The lesson type used in detail views
 */
export type LessonFull = components["schemas"]["LessonWithPhoto"];

export type UpsertPhoto = components["schemas"]["UpsertPhoto"];
