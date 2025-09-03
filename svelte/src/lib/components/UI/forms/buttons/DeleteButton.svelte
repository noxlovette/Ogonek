<script lang="ts">
  import { Trash2 } from "lucide-svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";

  const { confirmText, confirmTitle } = $props();
</script>

<UniButton
  variant="danger"
  formaction="?/delete"
  Icon={Trash2}
  ariaLabel="Delete"
  {confirmText}
  {confirmTitle}>{m.delete()}</UniButton
>
