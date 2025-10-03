<script lang="ts">
  let { badge = 0 } = $props();
  const badgeVisible = $derived(badge && badge != 0 ? true : false);
</script>

{#if badgeVisible}
  <div
    class="absolute -top-1 -right-1
           flex h-5 min-w-5 items-center justify-center
           rounded-full border border-red-400/20 bg-red-500
           px-1.5 text-xs font-bold text-white
           md:relative md:top-0 md:right-0 md:ml-2
           md:h-6 md:min-w-6"
  >
    {badge > 99 ? "99+" : badge}
  </div>
{/if}
