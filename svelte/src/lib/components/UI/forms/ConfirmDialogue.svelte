<script lang="ts">
  import { Ban, Trash2 } from "lucide-svelte";
  import { Body } from "../../typography";
  import Title1 from "../../typography/Title1.svelte";
  import ModalBackGround from "./ModalBackGround.svelte";
  import UniButton from "./UniButton.svelte";
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
  <HStack>
    <Title1>
      {confirmTitle}
    </Title1>

    <Body>
      {confirmText}
    </Body>

    <VStack>
      <UniButton Icon={Ban} onclick={() => (showConfirmDialog = false)}>
        Cancel
      </UniButton>
      <UniButton {formaction} Icon={Trash2} type="submit" variant="danger"
        >Confirm</UniButton
      >
    </VStack>
  </HStack>
</ModalBackGround>
