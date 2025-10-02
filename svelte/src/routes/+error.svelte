<script lang="ts">
  import { page } from "$app/state";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import {
    House,
    Send,
    ServerCrash,
    Bug,
    ShieldBan,
    FileSearch,
    Icon,
  } from "@lucide/svelte";
  import { onMount } from "svelte";

  type ErrorCode = 404 | 403 | 500;
  type ErrorConfig = {
    [key in ErrorCode]: {
      title: string;
      message: string;
      icon: typeof Icon;
      color: string;
    };
  } & {
    default: {
      title: string;
      message: string;
      icon: typeof Icon;
      color: string;
    };
  };

  const errorConfig: ErrorConfig = {
    404: {
      title: "Страница не найдена",
      message: "Ресурс не существует или был перемещён.",
      icon: FileSearch,
      color: "text-amber-600 dark:text-amber-400",
    },
    403: {
      title: "Доступ запрещён",
      message: "У вас недостаточно прав для просмотра содержимого.",
      icon: ShieldBan,
      color: "text-rose-600 dark:text-rose-400",
    },
    500: {
      title: "Внутренняя ошибка сервера",
      message: "Произошла непредвиденная ошибка. Инженеры уведомлены.",
      icon: ServerCrash,
      color: "text-rose-600 dark:text-rose-400",
    },
    default: {
      title: `Ошибка ${page.status}`,
      message: page.error?.message || "Произошла неожиданная ошибка.",
      icon: Bug,
      color: "text-stone-600 dark:text-stone-400",
    },
  };

  const currentError =
    errorConfig[page.status as ErrorCode] ?? errorConfig.default;

  const devFacts = [
    "🐛 Первый компьютерный баг был настоящей молью в 1947 году",
    "📊 В среднем — 70 багов на 1000 строк кода",
    "🎯 10% багов вызывают 90% крашей",
    "⏰ Половина рабочего времени уходит на отладку",
    "🌐 HTTP 404 якобы от комнаты №404 в CERN",
    "☕ Чем больше кофе — тем меньше багов",
    "🚀 В NASA код проходит 11 стадий ревью",
  ];

  let selectedFact: string;
  const errorId = page.error?.errorID;

  onMount(() => {
    selectedFact = devFacts[Math.floor(Math.random() * devFacts.length)];
  });
</script>

<div
  class="flex min-h-[70vh] w-full flex-col items-center justify-center px-4 py-16"
  in:fade={{ duration: 300, delay: 150 }}
>
  <div class="ring-default bg-default w-full max-w-md rounded-2xl">
    <!-- Header -->
    <div
      class="flex flex-col items-center space-y-4 bg-gradient-to-br from-stone-50 to-stone-100 p-8 dark:from-stone-800 dark:to-stone-700"
      in:fly={{ y: -20, duration: 400, delay: 200, easing: backOut }}
    >
      <div class="rounded-full bg-white/80 p-6 shadow-sm dark:bg-stone-800/80">
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

    <!-- Body -->
    <div class="gap-default p-8">
      <!-- Error ID -->
      {#if errorId}
        <div
          class="rounded-lg border border-stone-200 bg-stone-50 p-4 dark:border-stone-700 dark:bg-stone-800"
          in:fly={{ y: 20, duration: 400, delay: 300 }}
        >
          <p
            class="mb-1 text-sm font-medium text-stone-700 dark:text-stone-300"
          >
            ID ошибки:
          </p>
          <code
            class="rounded border bg-white px-2 py-1 font-mono text-sm dark:bg-stone-900"
          >
            {errorId}
          </code>
        </div>
      {/if}

      <!-- Dev fact -->
      {#if selectedFact}
        <div
          class="border-accent/20 from-accent/5 to-accent/10 dark:from-accent/5 dark:to-accent/10 rounded-lg border bg-gradient-to-r p-4"
          in:fly={{ y: 20, duration: 400, delay: 400 }}
        >
          <p class="text-sm leading-relaxed text-stone-700 dark:text-stone-300">
            {selectedFact}
          </p>
        </div>
      {/if}

      <!-- Actions -->
      <div
        class="flex flex-col gap-3 pt-2"
        in:fly={{ y: 20, duration: 400, delay: 500 }}
      >
        <UniButton
          type="button"
          variant="primary"
          Icon={House}
          content="На главную"
          iconOnly={false}
          href={`/${page.params.role || "s"}/dashboard`}
        ></UniButton>

        <UniButton
          content="Поддержка"
          type="button"
          iconOnly={false}
          Icon={Send}
          href="https://t.me/noxlovette"
        ></UniButton>
      </div>

      <!-- Footer -->
      <div
        class="border-t border-stone-200 pt-4 text-center dark:border-stone-700"
      >
        <p class="text-xs text-stone-500 dark:text-stone-500">
          При повторении ошибки укажите её ID в обращении.
        </p>
      </div>
    </div>
  </div>
</div>
