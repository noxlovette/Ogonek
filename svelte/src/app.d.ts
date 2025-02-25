// See https://kit.svelte.dev/docs/types#app

import type { JWTPayload } from "jose";

// for information about these interfaces
declare global {
  namespace App {
    interface Error {
      message: string;
      errorId?: number;
      status?: number;
    }

    interface Locals {
      accessToken?: string;
      refreshToken?: string;
      user?: JWTPayload & {
        sub: string;
        name?: string;
        role: string;
        email?: string;
        verified: boolean;
      };
    }
    interface Window {
      turnstile: {
        render: (element: HTMLElement, options: any) => string;
        remove: (widgetId: string) => void;
        reset: (widgetId: string) => void;
      };
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
