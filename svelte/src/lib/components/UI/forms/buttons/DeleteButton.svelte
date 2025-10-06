<script lang="ts">
  import { Shredder } from "@lucide/svelte";
  import { UniButton } from "..";
  import texts from "$lib/texts";
</script>

<UniButton
  variant="danger"
  formaction="?/delete"
  content={texts.crud.delete}
  ariaLabel="Delete"
  Icon={Shredder}
  shouldConfirm={true}
/>
