<script lang="ts">
  import { Caption1 } from "$lib/components/typography";
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";
  import UniButton from "../../forms/buttons/UniButton.svelte";
  import { currentPage } from "$lib/stores";
  import { Merger } from "../../toolbar";

  const { page, count, perPage, totalPages } = $props();
  const isFirstPage = $derived(page === 1);

  const isLastPage = $derived(page >= totalPages);
</script>

<div class="flex items-end justify-between px-4 py-3">
  <Caption1>
    Показано {(page - 1) * perPage + 1}–{Math.min(page * perPage, count)} из {count}
  </Caption1>

  {#if count > perPage}
    <Merger>
      <UniButton
        content="Предыдущая"
        Icon={ChevronLeft}
        disable={isFirstPage}
        onclick={() => currentPage.decrease()}
      />

      <UniButton
        content="Следующая"
        Icon={ChevronRight}
        disable={isLastPage}
        onclick={() => {
          currentPage.increase();
        }}
      />
    </Merger>
  {/if}
</div>
