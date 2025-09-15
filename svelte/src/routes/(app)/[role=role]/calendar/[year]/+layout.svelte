<script lang="ts">
  import { page } from "$app/state";
  import { UniButton } from "$lib/components";
  import { X } from "lucide-svelte";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();
</script>

<div
  class="bg-default ring-default absolute inset-y-0 right-0 w-1/2 gap-2 overflow-y-auto rounded-lg p-4 pb-8 shadow-xl md:gap-3 lg:gap-4"
>
  <UniButton
    href="/{page.params.role}/calendar"
    Icon={X}
    styling="absolute top-1 right-1 z-50"
  />
  {@render children()}
</div>
