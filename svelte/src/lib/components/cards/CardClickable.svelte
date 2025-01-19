<script lang="ts">
	let { href, children, pointerEventsOff = false } = $props();
</script>

<a
	{href}
	class:pointer-events-none={pointerEventsOff}
	class="relative flex flex-col p-5 rounded-xl
      bg-white
      ring-1 ring-milk-200
      shadow-lg shadow-milk-100
      hover:shadow-brick-100
      hover:ring-brick-400
      group transition-all duration-200 ease-out
      max-h-[150px] overflow-hidden"
>
	{@render children()}

	<div
		class="absolute bottom-0 left-0 right-0 h-12
    bg-gradient-to-t from-white to-transparent
    pointer-events-none"
	></div>
</a>
