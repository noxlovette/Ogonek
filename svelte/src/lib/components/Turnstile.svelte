<!-- Turnstile.svelte -->
<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { page } from "$app/state";
  import { Loader } from "lucide-svelte";
  import logger from "$lib/logger";

  let turnstileElement: HTMLDivElement;
  let widgetId: string | null = null;
  let isLoading = $state(true);
  let hasError = $state(false);
  let turnstileToken = $state<string>("");

  function initTurnstile() {
    isLoading = true;
    hasError = false;
    turnstileToken = "";

    if (typeof window === "undefined" || !window.turnstile) {
      const checkTurnstile = setInterval(() => {
        if (window.turnstile) {
          clearInterval(checkTurnstile);
          renderWidget();
        }
      }, 100);

      setTimeout(() => {
        clearInterval(checkTurnstile);
        if (!window.turnstile) {
          hasError = true;
          isLoading = false;
        }
      }, 5000);
      return;
    }

    renderWidget();
  }

  function renderWidget() {
    try {
      if (widgetId && window.turnstile) {
        window.turnstile.remove(widgetId);
      }

      if (window.turnstile && turnstileElement) {
        widgetId = window.turnstile.render(turnstileElement, {
          sitekey: "0x4AAAAAAA6Es9VtsFFGCAbw",
          theme: "auto",
          callback: (token: string) => {
            // Update reactive state instead of DOM manipulation
            turnstileToken = token;
          },
          "error-callback": () => {
            hasError = true;
            turnstileToken = "";
          },
        });
      }
    } catch (error) {
      logger.error("Turnstile initialization failed:", error);
      hasError = true;
    } finally {
      isLoading = false;
    }
  }

  // Reinit on route changes
  $effect(() => {
    if (page.url.pathname) {
      tick();
      setTimeout(initTurnstile, 0);
    }
  });

  onMount(() => {
    initTurnstile();
  });

  onDestroy(() => {
    if (widgetId && window.turnstile) {
      window.turnstile.remove(widgetId);
    }
  });
</script>

<div class="relative">
  {#if isLoading}
    <div class="absolute inset-0 flex items-center justify-center">
      <Loader class="font-stone-200 animate-spin"></Loader>
    </div>
  {/if}

  <div
    bind:this={turnstileElement}
    class="cf-turnstile my-4 duration-200"
    class:opacity-0={isLoading}
  ></div>

  <!-- Svelte-managed hidden input instead of DOM manipulation -->
  {#if turnstileToken}
    <input type="hidden" name="cf-turnstile-response" value={turnstileToken} />
  {/if}

  {#if hasError}
    <div class="text-brick-500 mt-2 text-sm">
      Failed to load security check. Please refresh the page.
    </div>
  {/if}
</div>
