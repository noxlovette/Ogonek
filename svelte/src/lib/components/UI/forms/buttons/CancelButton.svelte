<script lang="ts">
  import { Ban } from "@lucide/svelte";
  import { UniButton } from "..";
  import texts from "$lib/texts";
</script>

<UniButton content={texts.crud.cancel} ariaLabel="Cancel" Icon={Ban} href="."
></UniButton>
