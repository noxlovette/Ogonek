<script lang="ts">
  let {
    value = $bindable(),
    name = "completed",
    title = "Completed",
  } = $props();
</script>

<label class="relative inline-flex cursor-pointer items-center">
  <input type="checkbox" {name} bind:checked={value} class="peer sr-only" />
  <div
    class="peer peer-checked:bg-cacao-600 h-6 w-11 rounded-full bg-stone-200
       after:absolute after:top-[2px] after:left-[2px] after:h-5 after:w-5
       after:rounded-full after:border after:border-stone-300 after:bg-white
       after:transition-all after:content-[''] peer-checked:after:translate-x-full
       peer-checked:after:border-white
       dark:bg-stone-700/30 dark:peer-checked:bg-stone-500/30
       dark:after:border-stone-600"
  ></div>
  <span class="ml-3 text-sm font-medium text-stone-700 dark:text-stone-300">
    {title}
  </span>
</label>
