<script lang="ts">
	let props = $props();
	let activeIndex = $state(-1);
	let startY = 0;
	const swipeThreshold = 50;

	const hapticFeedback = {
		light: () => navigator.vibrate?.(10),
		medium: () => navigator.vibrate?.(15),
		heavy: () => navigator.vibrate?.([20, 10, 20])
	};

	function handleTouchStart(e: TouchEvent, i: number) {
		activeIndex = i;
		startY = e.touches[0].clientY;
		hapticFeedback.light();
	}

	function handleTouchMove(e: TouchEvent) {
		if (activeIndex === -1) return;

		const currentY = e.touches[0].clientY;
		const diff = startY - currentY;

		if (diff > swipeThreshold) {
			hapticFeedback.heavy();
			window.dispatchEvent(
				new CustomEvent('dockSwipeUp', {
					detail: { index: activeIndex }
				})
			);
		}
	}

	function handleTouchEnd() {
		if (activeIndex !== -1) hapticFeedback.medium();
		activeIndex = -1;
	}
</script>

<div
	class="{props.class} fixed flex items-center left-1/2 -translate-x-1/2 bottom-3 w-11/12 bg-cacao-600 backdrop-blur-lg text-cacao-50 dark:text-inherit dark:bg-milk-900 md:hidden px-2 py-4 ring-2 ring-milk-200 dark:ring-milk-900 rounded-2xl shadow-xl z-10"
>
	<ul class="{props.subclass} flex w-full justify-around items-end">
		{#each props.elements as Element, i}
			<li
				ontouchstart={(e) => handleTouchStart(e, i)}
				ontouchmove={handleTouchMove}
				ontouchend={handleTouchEnd}
				class="transition-all duration-300 ease-out {activeIndex === i ? 'scale-125' : 'scale-100'}"
			>
				<Element />
			</li>
		{/each}
	</ul>
</div>

<style>
	li {
		transform-origin: bottom;
		touch-action: none;
	}
</style>
