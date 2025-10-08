import type { JWTPayload } from "jose";

declare global {
  namespace App {
    interface Error {
      message: string;
      errorID: string;
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

    //interface PageData {}
    interface PageState {
      showImportModal?: boolean;
      searchOverlay?: boolean;
    }
    // interface Platform {}
  }
}

export {};
