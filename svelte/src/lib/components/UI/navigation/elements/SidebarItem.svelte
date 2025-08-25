<script lang="ts">
  import { page } from "$app/state";
  import { SquareArrowUpRight } from "lucide-svelte";
  import { Headline } from "$lib/components/typography";
  import NotificationBadge from "../../NotificationBadge.svelte";

  let {
    Icon,
    href = "/",
    name,
    external = false,
    badge = 0,
    disabled = false,
  }: {
    Icon?: any;
    href: string;
    name: string;
    external?: boolean;
    badge?: number;
    disabled?: boolean;
  } = $props();

  const target = $derived(external ? "_blank" : undefined);
  const rel = $derived(external ? "noopener noreferrer" : undefined);
  const isActive = $derived(
    page.url.pathname === href ||
      (href !== "/" && page.url.pathname.startsWith(href)),
  );

  const baseClasses =
    "group relative p-1 px-2 hidden md:flex items-center gap-2 font-medium rounded-2xl";

  const iconClasses = $derived(() => {
    return `size-5 ${isActive ? "text-accent" : "text-stone-500 dark:text-stone-400"}`;
  });
</script>

<a
  {href}
  {target}
  {rel}
  class={`${baseClasses}
      ${isActive ? "bg-accent text-stone-50" : ""}
    `}
  class:pointer-events-none={disabled}
  role={disabled ? "button" : "link"}
  aria-disabled={disabled}
>
  <div class="relative">
    <Icon class={`${iconClasses} ${isActive ? "drop-shadow-sm" : ""}`} />
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
