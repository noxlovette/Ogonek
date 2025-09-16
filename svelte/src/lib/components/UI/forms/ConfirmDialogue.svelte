<script lang="ts">
  import { Ban, Trash2 } from "lucide-svelte";
  import { Body } from "../../typography";
  import Title1 from "../../typography/Title1.svelte";
  import ModalBackGround from "./ModalBackGround.svelte";
  import UniButton from "./buttons/UniButton.svelte";
  import VStack from "../VStack.svelte";
  import HStack from "../HStack.svelte";
  import { Merger } from "../toolbar";

  let {
    confirmTitle = "You are deleting an item",
    confirmText = "Are you sure you want to do this?",
    showConfirmDialog = $bindable(false),
    formaction = null,
  } = $props();
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
        variant="danger">Confirm</UniButton
      >
    </VStack>
  </Merger>
</div>
