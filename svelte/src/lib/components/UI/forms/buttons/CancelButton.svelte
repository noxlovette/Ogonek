<script lang="ts">
  import { Ban } from "lucide-svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton ariaLabel="Cancel" Icon={Ban} href=".">{m.cancel()}</UniButton>
