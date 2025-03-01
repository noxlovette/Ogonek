// lib/utils/enhanceForm.ts
import { goto } from "$app/navigation";
import { isLoading } from "$lib/stores";
import { notification } from "$lib/stores/notification";

type MessageConfig = {
  success?: string;
  redirect?: string;
  failure?: string;
  error?: string;
  defaultError?: string;
};

type HandlerConfig = {
  success?: (result: any) => Promise<void> | void;
  redirect?: (result: any) => Promise<void> | void;
  failure?: (result: any) => Promise<void> | void;
  error?: (result: any) => Promise<void> | void;
};

type EnhanceConfig = {
  messages?: MessageConfig;
  handlers?: HandlerConfig;
  navigate?: boolean | string;
  shouldUpdate?: boolean;
};

export function enhanceForm(config: EnhanceConfig = {}) {
  const {
    messages = {},
    handlers = {},
    navigate = false,
    shouldUpdate = true,
  } = config;

  return ({ formElement, formData, action, cancel, submitter }) => {
    // Start loading
    isLoading.true();

    return async ({ result, update }) => {
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
          if (navigate === true && result.location) {
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
