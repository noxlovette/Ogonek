// See https://kit.svelte.dev/docs/types#app

import type { JWTPayload } from "jose";

// for information about these interfaces
declare global {
  namespace App {
    interface Error {
      message: string;
      errorId?: number;
      code?: number;
    }

    interface Locals {
      accessToken?: string;
      refreshToken?: string;
      user?: JWTPayload & {
        sub: string;
        name?: string;
        role: string;
      };
    }

    // interface PageData {}
    interface PageState {
      showImportModal?: boolean;
      searchOverlay?: boolean;
    }
    // interface Platform {}
  }
}

export {};
