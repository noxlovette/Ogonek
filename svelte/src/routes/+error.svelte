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
  import { Body, Callout, Merger, Title1 } from "$lib/components";

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
      color: "text-red-600 dark:text-red-400",
    },
    500: {
      title: "Внутренняя ошибка сервера",
      message: "Произошла непредвиденная ошибка. Инженеры уведомлены.",
      icon: ServerCrash,
      color: "text-red-600 dark:text-red-400",
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

<div class="mx-auto my-auto flex items-center justify-center text-center">
  <div class="gap-default w-full max-w-md rounded-2xl">
    <div class="bg-solid flex flex-col items-center space-y-4 p-3.5">
      <div class="rounded-full bg-white/80 p-6 shadow-sm dark:bg-stone-800/80">
        <svelte:component
          this={currentError.icon}
          class="h-16 w-16 {currentError.color}"
          stroke-width="1.5"
        />
      </div>
      <Title1>
        {currentError.title}
      </Title1>
      <Callout>
        {currentError.message}
      </Callout>
    </div>

    <div class="gap-default flex flex-col p-3.5">
      {#if errorId}
        <div
          class="rounded-lg border border-stone-200 bg-stone-50 p-3.5 dark:border-stone-700 dark:bg-stone-800"
        >
          <p
            class="mb-1 text-sm font-medium text-stone-700 dark:text-stone-300"
          >
            ID ошибки:
          </p>
          <code
            class="rounded border bg-white px-2 py-1 font-mono text-sm select-text dark:bg-stone-900"
          >
            {errorId}
          </code>
        </div>
      {/if}

      {#if selectedFact}
        <div class="border-accent bg-accent/12 rounded-lg border p-3.5">
          <Body>
            {selectedFact}
          </Body>
        </div>
      {/if}

      <!-- Actions -->
      <Merger>
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
      </Merger>
    </div>
  </div>
</div>
