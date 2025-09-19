<script lang="ts">
  import { page } from "$app/state";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import {
    House,
    Send,
    AlertTriangle,
    Server,
    FileQuestion,
  } from "lucide-svelte";
  import { onMount } from "svelte";

  const is404 = page.status === 404;
  const is500 = page.status >= 500;
  const is403 = page.status === 403;

  // Professional error messages in Russian
  const errorConfig = {
    404: {
      title: "Страница не найдена",
      message: "Запрашиваемый ресурс не существует или был перемещён.",
      icon: FileQuestion,
      color: "text-amber-600 dark:text-amber-400",
    },
    403: {
      title: "Доступ запрещён",
      message: "У вас недостаточно прав для просмотра этого содержимого.",
      icon: AlertTriangle,
      color: "text-red-600 dark:text-red-400",
    },
    500: {
      title: "Внутренняя ошибка сервера",
      message: "Произошла непредвиденная ошибка. Наши инженеры уже уведомлены.",
      icon: Server,
      color: "text-red-600 dark:text-red-400",
    },
    default: {
      title: `Ошибка ${page.status}`,
      message: page.error?.message || "Произошла неожиданная ошибка.",
      icon: AlertTriangle,
      color: "text-stone-600 dark:text-stone-400",
    },
  };

  const currentError = errorConfig[page.status] || errorConfig.default;

  // Dev humor facts in Russian - cached to prevent re-renders
  const devFacts = [
    "🐛 Первый компьютерный баг был настоящей молью, найденной в реле в 1947 году",
    "📊 В среднем разработчик создаёт 70 багов на 1000 строк кода",
    "🎯 10% багов ответственны за 90% всех крашей системы",
    "⏰ Большинство разработчиков тратят 50% времени на дебаг кода",
    "🌐 HTTP 404 якобы происходит от комнаты 404 в CERN, где стояли веб-серверы",
    "☕ Количество выпитого кофе прямо пропорционально количеству исправленных багов",
    "🚀 В NASA код проходит в среднем 11 стадий ревью перед продакшеном",
  ];

  let selectedFact: string;
  let errorId: string;

  onMount(() => {
    // Generate error ID for support
    errorId =
      `ERR-${Date.now()}-${Math.random().toString(36).substr(2, 5)}`.toUpperCase();

    // Select random fact once on mount
    selectedFact = devFacts[Math.floor(Math.random() * devFacts.length)];

    // Log error for monitoring (replace with your error tracking service)
    if (typeof window !== "undefined" && page.status >= 400) {
      console.warn(`Error ${page.status} encountered:`, {
        errorId,
        url: page.url.pathname,
        userAgent: navigator.userAgent,
        timestamp: new Date().toISOString(),
      });
    }
  });

  const handleSupportContact = () => {
    // Copy error details to clipboard for easier support
    if (navigator.clipboard) {
      const errorDetails = `Error ID: ${errorId}\nStatus: ${page.status}\nURL: ${page.url.pathname}\nTime: ${new Date().toLocaleString("ru-RU")}`;
      navigator.clipboard.writeText(errorDetails);
    }
  };
</script>

<div
  class="flex min-h-[70vh] w-full flex-col items-center justify-center px-4 py-16"
  in:fade={{ duration: 300, delay: 150 }}
>
  <div
    class="w-full max-w-lg overflow-hidden rounded-xl bg-white shadow-lg ring-1 ring-stone-200/50 dark:bg-stone-900 dark:ring-stone-700/50"
  >
    <!-- Header with icon -->
    <div
      class="flex w-full items-center justify-center bg-gradient-to-br from-stone-50 to-stone-100 p-8 dark:from-stone-800 dark:to-stone-700"
      in:fly={{ y: -20, duration: 400, delay: 200, easing: backOut }}
    >
      <div class="flex flex-col items-center space-y-4">
        <div
          class="rounded-full bg-white/80 p-6 shadow-sm dark:bg-stone-800/80"
        >
          <svelte:component
            this={currentError.icon}
            class="h-16 w-16 {currentError.color}"
            stroke-width="1.5"
          />
        </div>
        <div class="text-center">
          <h1 class="text-3xl font-bold text-stone-900 dark:text-stone-100">
            {currentError.title}
          </h1>
          <p class="mt-2 text-lg text-stone-600 dark:text-stone-400">
            {currentError.message}
          </p>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div class="space-y-6 p-8">
      <!-- Error ID for support -->
      {#if errorId}
        <div
          class="rounded-lg border border-stone-200 bg-stone-50 p-4 dark:border-stone-700 dark:bg-stone-800"
          in:fly={{ y: 20, duration: 400, delay: 300 }}
        >
          <p
            class="mb-1 text-sm font-medium text-stone-700 dark:text-stone-300"
          >
            ID ошибки для поддержки:
          </p>
          <code
            class="text-accent rounded border bg-white px-2 py-1 font-mono text-sm dark:bg-stone-900"
          >
            {errorId}
          </code>
        </div>
      {/if}

      <!-- Dev humor section -->
      {#if selectedFact}
        <div
          class="from-accent/5 to-accent/10 border-accent/20 dark:from-accent/5 dark:to-accent/10 rounded-lg border bg-gradient-to-r p-4"
          in:fly={{ y: 20, duration: 400, delay: 400 }}
        >
          <div class="flex items-start space-x-3">
            <div class="mt-0.5 flex-shrink-0">
              <div class="bg-accent h-2 w-2 rounded-full"></div>
            </div>
            <p
              class="text-sm leading-relaxed text-stone-700 dark:text-stone-300"
            >
              {selectedFact}
            </p>
          </div>
        </div>
      {/if}

      <!-- Action buttons -->
      <div
        class="flex flex-col gap-3 pt-2"
        in:fly={{ y: 20, duration: 400, delay: 500 }}
      >
        <UniButton
          type="button"
          variant="primary"
          iconOnly={false}
          Icon={House}
          href={`/${page.params.role || "s"}/dashboard`}
        >
          Вернуться на главную
        </UniButton>

        <UniButton
          type="button"
          Icon={Send}
          href="https://t.me/noxlovette"
          iconOnly={false}
          onClick={handleSupportContact}
        >
          Связаться с поддержкой
        </UniButton>
      </div>

      <!-- Additional help text -->
      <div
        class="border-t border-stone-200 pt-4 text-center dark:border-stone-700"
      >
        <p class="text-xs text-stone-500 dark:text-stone-500">
          Если проблема повторяется, приложите ID ошибки к обращению
        </p>
      </div>
    </div>
  </div>
</div>
