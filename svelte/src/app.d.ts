import type { AvailableLanguageTag } from "$lib/paraglide/runtime";
import type { ParaglideLocals } from "@inlang/paraglide-js";
import type { JWTPayload } from "jose";

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
      paraglide: ParaglideLocals<AvailableLanguageTag>;
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
