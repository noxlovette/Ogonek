// See https://kit.svelte.dev/docs/types#app

import type { JWTPayload } from "jose";

// for information about these interfaces
declare global {
  namespace App {
    interface Error {
      message: string;
      errorId: int;
    }
    interface Locals {
      accessToken?: string;
      refreshToken?: string;
      user?: JWTPayload;
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
