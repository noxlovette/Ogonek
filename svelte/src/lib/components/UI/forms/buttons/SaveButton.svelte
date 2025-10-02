<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { UniButton } from "..";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton
  content={m.save()}
  dataCy="save-button"
  variant="prominent"
  type="submit"
  ariaLabel="Save"
  Icon={Check}
></UniButton>
