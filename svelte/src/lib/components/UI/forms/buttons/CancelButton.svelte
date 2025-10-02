<script lang="ts">
  import { Ban } from "@lucide/svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton content={m.cancel()} ariaLabel="Cancel" Icon={Ban} href="."
></UniButton>
