<script lang="ts">
  import { Pencil } from "@lucide/svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";

  const { href } = $props();
</script>

<UniButton
  variant="prominent"
  content={m.edit()}
  dataCy="edit-button"
  type="submit"
  ariaLabel="Edit"
  Icon={Pencil}
  {href}
></UniButton>
