<script lang="ts">
  import { TickMorph, Toggler } from "$lib/components";
  import { notification } from "$lib/stores";
</script>

<button
  onclick={() =>
    notification.set({ message: "Hello, world!", type: "success" })}
>
  Show Success
</button>
<button
  onclick={() => notification.set({ message: "Hello, world!", type: "error" })}
>
  Show Error
</button>
<button
  onclick={() => notification.set({ message: "Hello, world!", type: "info" })}
>
  Show Info
</button>

<button
  onclick={() =>
    notification.set({
      message:
        "Enim et elit cupidatat consequat duis cillum nulla eiusmod ipsum est officia. Eu sit irure laborum enim commodo qui elit eiusmod. Officia velit exercitation exercitation laboris et pariatur qui aliquip deserunt labore laborum cupidatat sint esse minim.",
      type: "info",
    })}
>
  A really long message
</button>

<Toggler name="test" value={false}></Toggler>
<Toggler name="test" value={true}></Toggler>
<TickMorph name="test" value={false} />
<TickMorph name="test" value={true} />
