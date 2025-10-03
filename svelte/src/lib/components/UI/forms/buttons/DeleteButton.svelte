<script lang="ts">
  import { Shredder, Trash2 } from "@lucide/svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton
  variant="danger"
  formaction="?/delete"
  content={m.delete()}
  ariaLabel="Delete"
  Icon={Shredder}
  shouldConfirm={true}
/>
