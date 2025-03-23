<!-- Unauthorized.svelte -->
<script>
  import { Lock, ArrowLeft } from "lucide-svelte";
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
      Woah there, partner! 🤠
    </h1>

    <p class="mb-6 text-xl text-stone-600">
      Looks like you're trying to sneak into the VIP section without a backstage
      pass!
    </p>

    <div class="mb-8 rounded-lg bg-stone-50 p-4">
      <p class="text-stone-500 italic">
        "HTTP 403: The bouncer said no. Maybe try bringing cookies next time?
        🍪"
      </p>
    </div>

    <button
      class="group inline-flex items-center gap-2 rounded-lg bg-red-600 px-6 py-3 text-white transition-colors duration-200 hover:bg-red-700"
      onclick={() => window.history.back()}
    >
      <ArrowLeft
        class="group-hover:-transtone-x-1 transition-transform duration-200"
      />
      Return to Safety
    </button>
  </div>
</div>

<svelte:head>
  <title>Unauthorised</title>
</svelte:head>
