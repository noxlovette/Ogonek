import type { JWTPayload } from "jose";
import type { components } from "./gen/openapi";

export type User = components["schemas"]["User"];
export type Profile = components["schemas"]["Profile"];
export type Student = components["schemas"]["Student"];

export interface Claims extends JWTPayload {
  role: components["schemas"]["UserRole"];
}

export type UserRole = components["schemas"]["UserRole"];
