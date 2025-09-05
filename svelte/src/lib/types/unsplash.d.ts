import type { components } from "./api/gen/openapi";

export interface PhotoURLs {
  raw: string;
  full: string;
  regular: string;
  small: string;
  thumb: string;
}

export type Photo = components["schemas"]["Photo"];
