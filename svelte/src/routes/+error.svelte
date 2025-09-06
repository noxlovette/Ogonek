<script lang="ts">
  import { page } from "$app/state";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { House, Send } from "lucide-svelte";

  // Determine error category for custom messaging
  const is404 = page.status === 404;
  const is500 = page.status >= 500;

  const errorTitle = is404
    ? "Page Not Found"
    : is500
      ? "Server Error"
      : `Error ${page.status}`;

  const errorMessage = is404
    ? "We couldn't find the page you're looking for."
    : is500
      ? "Something went wrong on our end."
      : page.error?.message || "An unexpected error occurred.";

  // For fun error facts (randomly selected)
  const errorFacts = [
    "Did you know? The first computer bug was an actual moth found in a relay in 1947.",
    "The average developer creates 70 bugs per 1000 lines of code.",
    "Studies show 10% of bugs are responsible for 90% of all system crashes.",
    "Most developers spend 50% of their time debugging code.",
    "The term '404' is said to originate from room 404 at CERN where the web servers were kept.",
  ];

  const randomFact = errorFacts[Math.floor(Math.random() * errorFacts.length)];
</script>

<div
  class="flex min-h-[70vh] w-full flex-col items-center justify-center px-4 py-16"
  in:fade={{ duration: 300, delay: 150 }}
>
  <div
    class="w-full max-w-md overflow-hidden rounded-lg bg-white shadow-sm dark:bg-stone-800"
  >
    <div
      class="ring-b flex w-full justify-center bg-stone-100 p-6 ring-stone-300/40 dark:bg-stone-700 dark:ring-stone-600"
    >
      <div in:fly={{ y: -20, duration: 400, delay: 200, easing: backOut }}>
        {#if is404}
          <img
            src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExNnk3eTZ1NzBjc3l3cmFkY2g2bmRwbGl6Z3MwN2E3amg3YmNpbDVteiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/14uQ3cOFteDaU/giphy.gif"
            alt="Lost"
            class="h-48 rounded object-cover"
          />
        {:else if is500}
          <img
            src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExd3BpbTRtbHAwNzg2cnEwem04aXd4c2ZsNG15YXF1bGg3Y2Y1c2VpdiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/xT5LMBauOi9MgA50L6/giphy.gif"
            alt="Server error"
            class="h-48 rounded object-cover"
          />
        {:else}
          <img
            src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExaHF3Y3F5bHdtYXBnMGh0bHNxeXhnZnRldDFuOTA0c2VudHU0YnJpciZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/xT5LMzIK1AdZJ4cYW4/giphy.gif"
            alt="Confused"
            class="h-48 rounded object-cover"
          />
        {/if}
      </div>
    </div>

    <!-- Error details -->
    <div class="p-6">
      <div
        class="mb-4 text-center"
        in:fly={{ y: 20, duration: 400, delay: 300 }}
      >
        <h1 class="text-accent dark:text-accent text-2xl font-bold">
          {errorTitle}
        </h1>
        <p class="mt-2 text-stone-600 dark:text-stone-400">
          {errorMessage}
        </p>
      </div>

      <!-- Fun error fact -->
      <div
        class="bg-default mb-6 rounded-lg p-4 text-sm text-stone-700 dark:bg-stone-700/30 dark:text-stone-300"
        in:fly={{ y: 20, duration: 400, delay: 400 }}
      >
        <p class="flex items-start">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="text-accent mr-2 h-5 w-5 flex-shrink-0"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
              clip-rule="evenodd"
            />
          </svg>
          <span>{randomFact}</span>
        </p>
      </div>

      <!-- Action buttons -->
      <div class="flex flex-col gap-3">
        <UniButton
          type="button"
          variant="primary"
          iconOnly={false}
          Icon={House}
          href={`/${page.params.role || "s"}/dashboard`}
          >Back to Safety</UniButton
        >
        <UniButton
          type="button"
          Icon={Send}
          href="https://t.me/noxlovette"
          iconOnly={false}>Contact Support</UniButton
        >
      </div>
    </div>
  </div>
</div>
