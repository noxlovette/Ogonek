<script lang="ts">
  import { Save } from "@lucide/svelte";
  import { UniButton } from "..";
  import texts from "$lib/texts";
</script>

<UniButton
  content={texts.crud.save}
  dataCy="save-button"
  variant="prominent"
  type="submit"
  ariaLabel="Save"
  Icon={Save}
></UniButton>
