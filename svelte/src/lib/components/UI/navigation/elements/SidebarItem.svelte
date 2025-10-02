<script lang="ts">
  import { page } from "$app/state";
  import { SquareArrowUpRight } from "@lucide/svelte";
  import { Headline } from "$lib/components/typography";
  import { NotificationBadge } from "../../content";

  let {
    Icon,
    href = "/",
    name,
    external = false,
    badge = 0,
    disabled = false,
    dataCy = "",
  }: {
    Icon?: any;
    href: string;
    name: string;
    external?: boolean;
    badge?: number;
    disabled?: boolean;
    dataCy?: string;
  } = $props();

  const target = $derived(external ? "_blank" : undefined);
  const rel = $derived(external ? "noopener noreferrer" : undefined);
  const isActive = $derived(
    page.url.pathname === href ||
      (href !== "/" && page.url.pathname.startsWith(href)),
  );

  const baseClasses =
    "group relative transition-all duration-150 p-2 px-2.5 hidden md:flex items-center gap-2 font-medium rounded-2xl";

  const iconClasses = $derived(() => {
    return `size-5 ${isActive ? "text-accent" : "text-stone-500 dark:text-stone-400"}`;
  });
</script>

<a
  {href}
  {target}
  {rel}
  class={`${baseClasses}
      ${isActive ? " text-accent bg-stone-100/80 hover:bg-stone-200/50 dark:bg-stone-900/80 dark:hover:bg-stone-900/50" : "hover-default"}
    `}
  class:pointer-events-none={disabled}
  data-cy={dataCy}
  role={disabled ? "button" : "link"}
  aria-disabled={disabled}
>
  <div class="relative">
    <Icon class={`${iconClasses} ${isActive ? "" : ""}`} />
  </div>

  <div class="flex min-w-0 flex-1 items-center justify-between">
    <Headline>
      {name}
    </Headline>

    <NotificationBadge {badge} />

    {#if external}
      <div
        class="ml-2 flex h-4 w-4 items-center justify-center opacity-60 hover:opacity-100"
      >
        <SquareArrowUpRight />
      </div>
    {/if}
  </div>
</a>
