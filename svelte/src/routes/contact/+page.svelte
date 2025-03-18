<script lang="ts">
  import { Mail, MessageSquare } from "lucide-svelte";

  const contactInfo = {
    email: "danila.volkov@noxlovette.com",
    telegram: "@noxlovette",
  };

  let copied = $state(false);
  let copiedType = $state("");

  function copyToClipboard(text: string, type: string) {
    navigator.clipboard.writeText(text).then(() => {
      copied = true;
      copiedType = type;
      setTimeout(() => {
        copied = false;
      }, 2000);
    });
  }
</script>

<div class="mx-auto max-w-3xl px-4">
  <div class="mb-12 text-center">
    <h1 class="mb-4 text-4xl font-bold text-stone-800 dark:text-stone-100">
      Contact Me
    </h1>
    <p class="mx-auto max-w-xl text-stone-600 dark:text-stone-300">
      Need to get in touch? Feel free to reach out through email or Telegram.
    </p>
  </div>

  <div class="grid gap-6 md:grid-cols-2">
    <!-- Email Card -->
    <div
      class="overflow-hidden rounded-xl border border-stone-200 bg-white shadow-sm transition-all hover:shadow-lg dark:border-stone-700 dark:bg-stone-900"
    >
      <div class="p-6">
        <div class="mb-4 flex items-center">
          <div class="bg-cacao-100 dark:bg-cacao-900 mr-4 rounded-full p-3">
            <Mail class="text-cacao-600 dark:text-cacao-400 h-6 w-6" />
          </div>
          <h2 class="text-xl font-semibold text-stone-800 dark:text-stone-200">
            Email
          </h2>
        </div>

        <p class="mb-4 text-stone-600 dark:text-stone-400">
          Drop me an email and I'll get back to you as soon as possible.
        </p>

        <div
          class="flex items-center justify-between rounded-lg bg-stone-50 p-3 dark:bg-stone-800"
        >
          <span
            class="font-mono text-sm break-all text-stone-800 md:text-base dark:text-stone-300"
          >
            {contactInfo.email}
          </span>
          <button
            class="bg-cacao-500 hover:bg-cacao-600 ml-2 rounded-md px-3 py-1.5 text-sm text-white transition-colors"
            onclick={() => copyToClipboard(contactInfo.email, "email")}
          >
            {copied && copiedType === "email" ? "Copied!" : "Copy"}
          </button>
        </div>

        <a
          href={`mailto:${contactInfo.email}`}
          class="bg-cacao-500 hover:bg-cacao-600 mt-4 block w-full rounded-md py-3 text-center font-medium text-white transition-colors"
        >
          Send Email
        </a>
      </div>
    </div>

    <!-- Telegram Card -->
    <div
      class="overflow-hidden rounded-xl border border-stone-200 bg-white shadow-sm transition-all hover:shadow-lg dark:border-stone-700 dark:bg-stone-900"
    >
      <div class="p-6">
        <div class="mb-4 flex items-center">
          <div class="bg-cacao-100 dark:bg-cacao-900 mr-4 rounded-full p-3">
            <MessageSquare class="text-cacao-600 dark:text-cacao-400 h-6 w-6" />
          </div>
          <h2 class="text-xl font-semibold text-stone-800 dark:text-stone-200">
            Telegram
          </h2>
        </div>

        <p class="mb-4 text-stone-600 dark:text-stone-400">
          For faster responses, reach out to me on Telegram.
        </p>

        <div
          class="flex items-center justify-between rounded-lg bg-stone-50 p-3 dark:bg-stone-800"
        >
          <span
            class="font-mono text-sm text-stone-800 md:text-base dark:text-stone-300"
          >
            {contactInfo.telegram}
          </span>
          <button
            class="bg-cacao-500 hover:bg-cacao-600 ml-2 rounded-md px-3 py-1.5 text-sm text-white transition-colors"
            onclick={() => copyToClipboard(contactInfo.telegram, "telegram")}
          >
            {copied && copiedType === "telegram" ? "Copied!" : "Copy"}
          </button>
        </div>

        <a
          href={`https://t.me/${contactInfo.telegram.replace("@", "")}`}
          target="_blank"
          rel="noopener noreferrer"
          class="bg-cacao-500 hover:bg-cacao-600 mt-4 block w-full rounded-md py-3 text-center font-medium text-white transition-colors"
        >
          Message on Telegram
        </a>
      </div>
    </div>
  </div>

  <!-- Additional Contact Info or Form Could Go Here -->
  <div class="mt-12 text-center">
    <p class="text-stone-600 dark:text-stone-400">
      I typically respond within 24 hours during business days.
    </p>
  </div>
</div>
