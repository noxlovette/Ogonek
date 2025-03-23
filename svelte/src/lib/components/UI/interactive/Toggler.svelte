<script lang="ts">
  let { value = $bindable() } = $props();
</script>

<label class="relative inline-flex cursor-pointer items-center">
  <input
    type="checkbox"
    name="completed"
    bind:checked={value}
    class="peer sr-only"
  />
  <div
    class="peer peer-focus:ring-cacao-300 peer-checked:bg-cacao-600 h-6 w-11 rounded-full bg-stone-200 peer-focus:ring-4
       peer-focus:outline-none after:absolute after:top-[2px] after:left-[2px] after:h-5 after:w-5
       after:rounded-full after:border after:border-stone-300 after:bg-white
       after:transition-all after:content-[''] peer-checked:after:translate-x-full
       peer-checked:after:border-white
       dark:bg-stone-700 dark:peer-checked:bg-stone-500 dark:peer-focus:ring-stone-500
       dark:after:border-stone-600"
  ></div>
  <span class="ml-3 text-sm font-medium text-stone-700 dark:text-stone-300"
    >Completed</span
  >
</label>
