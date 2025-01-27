<script lang="ts">
	let { href, children } = $props();
</script>

<a
	{href}
	class="relative flex flex-col p-5 rounded-xl
      bg-white
      dark:bg-milk-950
      ring-1 ring-milk-200
      dark:ring-milk-900
      shadow-lg shadow-milk-100
      dark:shadow-milk-900
      hover:shadow-brick-100
      dark:hover:shadow-brick-800
      dark:hover:ring-brick-800
      hover:ring-brick-400
      group transition-all duration-200 ease-out
      h-[150px] overflow-hidden"
>
	{@render children()}

	<div
		class="absolute bottom-0 left-0 right-0 h-12
    bg-gradient-to-t dark:from-milk-950 from-white to-transparent
    pointer-events-none"
	></div>
</a>
