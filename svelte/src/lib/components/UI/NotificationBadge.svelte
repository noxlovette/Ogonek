<script lang="ts">
  let { badge = 0 } = $props();

  const badgeVisible = $derived(badge && badge != 0 ? true : false);
</script>

{#if badgeVisible}
  <div
    class="absolute -top-1 -right-1 h-4 animate-pulse items-center justify-center rounded-full border border-red-400/20 bg-red-500 px-2 text-xs font-bold
            text-white hover:animate-none
            md:relative md:-top-0 md:-right-0 md:ml-2
            md:flex md:h-6
            md:min-w-[24px] dark:bg-red-600
          "
  >
    {badge > 99 ? "99+" : badge}
  </div>
{/if}
