<script lang="ts">
    import { page } from "$app/stores";
  </script>
  
      <div class="p-4">
        <h1 class="text-4xl font-oswald">Error {$page.status}: {$page.error.message}</h1>
        <a
          href="https://t.me/noxlovette"
          class="text-6xl font-bold"
          >Nothing is perfect. Please complain by clicking on this message.</a
        >
      </div>