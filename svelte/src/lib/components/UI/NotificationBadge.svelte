<script lang="ts">
  let { badge = 0 } = $props();

  const badgeVisible = $derived(badge && badge != 0 ? true : false);
</script>

{#if badgeVisible}
  <div
    class="h-4flex absolute -top-1 -right-1 min-w-[24px] animate-pulse items-center justify-center rounded-full border border-red-400/20
            bg-red-500 px-2
            text-xs font-bold text-white hover:animate-none
            md:ml-2 md:block
            md:h-6 dark:bg-red-600
          "
  >
    {badge > 99 ? "99+" : badge}
  </div>
{/if}
