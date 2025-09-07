<script lang="ts">
  import { Pencil } from "lucide-svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";

  const { href } = $props();
</script>

<UniButton
  variant="prominent"
  type="submit"
  ariaLabel="Edit"
  Icon={Pencil}
  {href}>{m.edit()}</UniButton
>
