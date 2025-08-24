<script>
  import { isLoading } from "$lib/stores";
  import { Loader } from "lucide-svelte";
  import ModalBackGround from "./forms/ModalBackGround.svelte";
</script>

{#if $isLoading}
  <ModalBackGround soft={true}>
    <div class="bg-default animate-spin p-12"><Loader /></div>
  </ModalBackGround>
{/if}
