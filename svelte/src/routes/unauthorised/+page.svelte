<!-- Unauthorized.svelte -->
<script>
  import { DoorClosed, Lock, ShieldAlert, ArrowLeft } from "lucide-svelte";
</script>

<div class="flex size-full items-center justify-center">
  <div
    class="w-full max-w-2xl overflow-hidden rounded-lg bg-white p-8 text-center shadow-md"
  >
    <!-- Main content -->
    <div class="mb-6 flex justify-center space-x-4">
      <Lock class="text-milk-400 h-16 w-16" />
    </div>

    <h1 class="text-milk-800 mb-4 text-4xl font-bold">
      Woah there, partner! 🤠
    </h1>

    <p class="text-milk-600 mb-6 text-xl">
      Looks like you're trying to sneak into the VIP section without a backstage
      pass!
    </p>

    <div class="bg-milk-50 mb-8 rounded-lg p-4">
      <p class="text-milk-500 italic">
        "HTTP 403: The bouncer said no. Maybe try bringing cookies next time?
        🍪"
      </p>
    </div>

    <button
      class="group inline-flex items-center gap-2 rounded-lg bg-red-600 px-6 py-3 text-white transition-colors duration-200 hover:bg-red-700"
      onclick={() => window.history.back()}
    >
      <ArrowLeft
        class="group-hover:-tranmilk-x-1 transition-transform duration-200"
      />
      Return to Safety
    </button>
  </div>
</div>

<svelte:head>
  <title>Unauthorised</title>
</svelte:head>
