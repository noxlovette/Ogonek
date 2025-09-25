<script lang="ts">
  import { page } from "$app/state";
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { HStack } from "../..";
  import NotificationBadge from "../../content/NotificationBadge.svelte";
  let {
    Icon,
    href = "/",
    name,
    badge = 0,
  }: {
    Icon?: any;
    href: string;
    name: string;
    badge?: number;
  } = $props();

  const isActive = $derived(
    page.url.pathname === href ||
      (href !== "/" && page.url.pathname.startsWith(href)),
  );
</script>

<a {href} class="relative flex items-center p-2 md:hidden">
  <HStack styling="items-center">
    <div class="relative">
      <Icon
        class={isActive ? "text-accent" : "text-stone-500 dark:text-stone-400"}
      />
    </div>

    <Caption1 styling={isActive ? "text-accent" : ""}>
      {name}
    </Caption1>

    <NotificationBadge {badge} />
  </HStack>
</a>
