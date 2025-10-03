<script lang="ts">
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";
  import type { Snippet } from "svelte";
  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<form
  class="ring-default bg-solid rounded-2xl shadow-md"
  aria-label="Data table"
  method="POST"
  use:enhance={enhanceForm({ messages: { success: "Успех" } })}
>
  {@render children()}
</form>
