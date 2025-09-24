<script lang="ts">
  import { Trash2 } from "lucide-svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton
  variant="danger"
  formaction="?/delete"
  content={m.delete()}
  Icon={Trash2}
  ariaLabel="Delete"
  shouldConfirm={true}
/>
