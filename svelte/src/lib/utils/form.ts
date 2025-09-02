import { goto } from "$app/navigation";
import logger from "$lib/logger";
import { isLoading } from "$lib/stores";
import { notification } from "$lib/stores/notification";
import type { SubmitFunction } from "@sveltejs/kit";
import { fail, type ActionFailure } from "@sveltejs/kit";
import type { infer as ZodInfer, ZodObject, ZodRawShape } from "zod";
type ValidationResult<T> =
  | { success: true; data: T }
  | { success: false; failure: ActionFailure<{ message: string }> };

export function validateForm<T extends ZodRawShape>(
  formData: FormData,
  zodObject: ZodObject<T>,
): ValidationResult<ZodInfer<ZodObject<T>>> {
  const rawData = Object.fromEntries(formData);
  const validationResult = zodObject.safeParse(rawData);

  logger.debug(validationResult);

  if (!validationResult.success) {
    return {
      success: false,
      failure: fail(400, { message: validationResult.error.message }),
    };
  }

  return { success: true, data: validationResult.data };
}

/*
const validation = validateForm(formData, loginSchema);

if (!validation.success) return validation.failure;

const { email, password } = validation.data;

*/
// Define custom action result types that extend SvelteKit's built-in types
type ActionResultSuccess = {
  type: "success";
  status: number;
  data?: Record<string, any>;
};

type ActionResultRedirect = {
  type: "redirect";
  status: number;
  location: string;
};

type ActionResultFailure = {
  type: "failure";
  status: number;
  data?: Record<string, any>;
};

type ActionResultError = {
  type: "error";
  status?: number;
  error?: Error;
};

// Combined custom ActionResult type
type CustomActionResult =
  | ActionResultSuccess
  | ActionResultRedirect
  | ActionResultFailure
  | ActionResultError;

type MessageConfig = {
  success?: string;
  redirect?: string;
  failure?: string;
  error?: string;
  defaultError?: string;
};

type HandlerConfig = {
  success?: (result: ActionResultSuccess) => Promise<void> | void;
  redirect?: (result: ActionResultRedirect) => Promise<void> | void;
  failure?: (result: ActionResultFailure) => Promise<void> | void;
  error?: (result: ActionResultError) => Promise<void> | void;
};

type EnhanceConfig = {
  messages?: MessageConfig;
  handlers?: HandlerConfig;
  navigate?: boolean | string;
  shouldUpdate?: boolean;
};

export function enhanceForm(config: EnhanceConfig = {}): SubmitFunction {
  const {
    messages = {},
    handlers = {},
    navigate = false,
    shouldUpdate = true,
  } = config;

  return () => {
    // Start loading
    isLoading.true();

    return async ({
      result,
      update,
    }: {
      result: CustomActionResult;
      update: () => void;
    }) => {
      // End loading regardless of result
      isLoading.false();

      // Extract error message based on result type
      const getErrorMessage = () => {
        if (result.type === "failure" && result.data?.message) {
          return String(result.data.message);
        } else if (result.type === "error" && result.error?.message) {
          return String(result.error.message);
        }
        return messages.defaultError || "Something went wrong";
      };

      // Handle the result
      switch (result.type) {
        case "success":
          // Show success notification if provided
          if (messages.success) {
            notification.set({
              message: messages.success,
              type: "success",
            });
          }
          // Call success handler if provided
          if (handlers.success) {
            await handlers.success(result);
          }
          break;

        case "redirect":
          // Show redirect notification if provided
          if (messages.redirect) {
            notification.set({
              message: messages.redirect,
              type: "success",
            });
          }
          // Call redirect handler if provided
          if (handlers.redirect) {
            await handlers.redirect(result);
          }
          // Update the form if requested
          if (shouldUpdate) {
            update();
          }
          // Navigate if requested
          if (navigate === true) {
            await goto(result.location);
          } else if (typeof navigate === "string") {
            await goto(navigate);
          }
          break;

        case "failure":
          // Show failure notification
          notification.set({
            message: messages.failure || getErrorMessage(),
            type: "error",
          });
          // Call failure handler if provided
          if (handlers.failure) {
            await handlers.failure(result);
          }
          break;

        case "error":
          // Show error notification
          notification.set({
            message: messages.error || getErrorMessage(),
            type: "error",
          });
          // Call error handler if provided
          if (handlers.error) {
            await handlers.error(result);
          }
          break;
      }
    };
  };
}
