<script lang="ts">
  import { Pencil } from "@lucide/svelte";
  import { UniButton } from "..";
  import texts from "$lib/texts";

  const { href } = $props();
</script>

<UniButton
  variant="prominent"
  content={texts.crud.edit}
  dataCy="edit-button"
  type="submit"
  ariaLabel="Edit"
  Icon={Pencil}
  {href}
></UniButton>
