<script lang="ts">
  import { Ban, Trash2 } from "lucide-svelte";
  import { Body } from "../../typography";
  import Title1 from "../../typography/Title1.svelte";
  import ModalBackGround from "./ModalBackGround.svelte";
  import UniButton from "./buttons/UniButton.svelte";
  import VStack from "../VStack.svelte";
  import HStack from "../HStack.svelte";

  let {
    confirmTitle = "You are deleting an item",
    confirmText = "Are you sure you want to do this?",
    showConfirmDialog = $bindable(false),
    formaction = null,
  } = $props();
</script>

<ModalBackGround>
  <div role="dialog" aria-label="Confirmation Dialogue" class="gap-2">
    <Title1>
      {confirmTitle}
    </Title1>

    <Body>
      {confirmText}
    </Body>

    <VStack>
      <UniButton
        Icon={Ban}
        iconOnly={false}
        onclick={() => (showConfirmDialog = false)}
        ariaLabel="Cancel Deletion"
      >
        Cancel
      </UniButton>
      <UniButton
        {formaction}
        Icon={Trash2}
        ariaLabel="Confirm"
        type="submit"
        iconOnly={false}
        variant="danger">Confirm</UniButton
      >
    </VStack>
  </div>
</ModalBackGround>
