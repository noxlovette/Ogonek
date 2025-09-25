<script lang="ts">
  import { Trash2 } from "lucide-svelte";
  import UniButton from "./buttons/UniButton.svelte";
  import VStack from "../layout/VStack.svelte";
  import { Merger } from "../toolbar";

  let { showConfirmDialog = $bindable(false), formaction = null } = $props();
</script>

<div
  role="dialog"
  aria-label="Confirmation Dialogue"
  class="absolute top-12 right-0 z-50"
>
  <Merger styling="bg-solid">
    <VStack>
      <UniButton
        {formaction}
        Icon={Trash2}
        ariaLabel="Confirm"
        type="submit"
        iconOnly={false}
        content="Подтвердить"
        variant="danger"
      ></UniButton>
    </VStack>
  </Merger>
</div>
