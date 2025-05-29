<script lang="ts">
  import { PlusCircle } from "lucide-svelte";

  let { href = ".", body = {} } = $props();
</script>

<form
  action={href}
  method="post"
  class="hover:bg-cacao-600 hover:text-cacao-50 flex max-h-[150px] flex-col items-center justify-center rounded-lg px-5 py-3 shadow-sm ring ring-stone-300/40 transition-colors"
>
  <button class="flex size-full items-center justify-center">
    <input type="hidden" name="body" value={body} />
    <PlusCircle class="size-12" />
  </button>
</form>
