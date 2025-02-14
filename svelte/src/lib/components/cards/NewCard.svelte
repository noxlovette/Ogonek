<script lang="ts">
  import { PlusCircle } from "lucide-svelte";
  import CardClickable from "./CardClickable.svelte";

  let { href = ".", body = {} } = $props();
</script>

<form
  action={href}
  method="post"
  class="hover:bg-cacao-600 hover:text-cacao-50 ring-milk-200 flex max-h-[150px] flex-col items-center justify-center rounded-lg px-5 py-3 ring-2 shadow-md transition-colors"
>
  <button class="flex size-full items-center justify-center">
    <input type="hidden" name="body" value={body} />
    <PlusCircle class="size-12" />
  </button>
</form>
