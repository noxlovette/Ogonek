<!-- New SettingsRow component -->
<script lang="ts">
  let { label = "", children, description = "" } = $props();
</script>

<div class="flex items-center justify-between py-3 first:pt-0 last:pb-0">
  <div class="min-w-0 flex-1">
    <div class="text-sm font-medium text-stone-900 dark:text-stone-100">
      {label}
    </div>
    {#if description}
      <div class="mt-0.5 text-xs text-stone-500 dark:text-stone-400">
        {description}
      </div>
    {/if}
  </div>
  <div class="ml-4 flex-shrink-0">
    {@render children?.()}
  </div>
</div>
