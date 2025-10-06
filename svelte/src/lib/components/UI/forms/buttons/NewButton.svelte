<script>
  import { Plus } from "@lucide/svelte";
  import { UniButton } from ".";
  import texts from "$lib/texts";
</script>

<UniButton
  ariaLabel="Add"
  content={texts.crud.create}
  Icon={Plus}
  type="submit"
  variant="primary"
></UniButton>
