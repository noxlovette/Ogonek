import type { components } from "./gen/openapi";

export type ActivityLog = components["schemas"]["ActivityLog"];
export type UserPreferences = components["schemas"]["UserPreferences"];
export type AppContext = components["schemas"]["AppContext"];
export type DashboardData = components["schemas"]["DashboardData"];
export type NotificationBadges = components["schemas"]["NotificationBadges"];
export type CompositeStudent = components["schemas"]["CompositeStudent"];

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  perPage: number;
}
