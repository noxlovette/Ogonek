<script>
  import { Plus } from "@lucide/svelte";
  import { UniButton } from ".";
  import { m } from "$lib/paraglide/messages";
</script>

<UniButton
  ariaLabel="Add"
  content={m.new()}
  Icon={Plus}
  type="submit"
  variant="primary"
></UniButton>
