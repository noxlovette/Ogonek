<script lang="ts">
  import { Lock, ArrowLeft } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<div class="flex size-full items-center justify-center">
  <div
    class="w-full max-w-2xl overflow-hidden rounded-lg bg-white p-8 text-center shadow-sm"
  >
    <!-- Main content -->
    <div class="mb-6 flex justify-center space-x-4">
      <Lock class="h-16 w-16 text-stone-400" />
    </div>

    <h1 class="mb-4 text-4xl font-bold text-stone-800">
      {m.noisy_such_bear_achieve()}
    </h1>

    <div class="bg-default mb-8 rounded-lg p-4">
      <p class="text-stone-500 italic">
        {m.stale_vivid_dolphin_thrive()}
      </p>
    </div>

    <button
      class="group inline-flex items-center gap-2 rounded-lg bg-red-600 px-6 py-3 text-stone-50 hover:bg-red-700"
      onclick={() => window.history.back()}
    >
      <ArrowLeft class="group-hover:-transtone-x-1  " />
      {m.level_low_angelfish_peel()}
    </button>
  </div>
</div>

<svelte:head>
  <title>{m.brave_fair_cobra_tear()}</title>
</svelte:head>
