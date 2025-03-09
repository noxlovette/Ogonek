<script lang="ts">
  let props = $props();
  let activeIndex = $state(-1);
  let startY = 0;
  const swipeThreshold = 50;

  const hapticFeedback = {
    light: () => navigator.vibrate?.(10),
    medium: () => navigator.vibrate?.(15),
    heavy: () => navigator.vibrate?.([20, 10, 20]),
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
        new CustomEvent("dockSwipeUp", {
          detail: { index: activeIndex },
        }),
      );
    }
  }

  function handleTouchEnd() {
    if (activeIndex !== -1) hapticFeedback.medium();
    activeIndex = -1;
  }
</script>

<div
  class="{props.class} bg-cacao-600 text-cacao-50 fixed bottom-3 left-1/2 z-10 flex w-11/12 -translate-x-1/2 items-center rounded-2xl px-2 py-4 shadow-xl ring ring-stone-200 backdrop-blur-lg md:hidden dark:bg-stone-900 dark:text-inherit dark:ring-stone-900"
>
  <ul class="{props.subclass} flex w-full items-end justify-around">
    {#each props.elements as Element, i}
      <li
        ontouchstart={(e) => handleTouchStart(e, i)}
        ontouchmove={handleTouchMove}
        ontouchend={handleTouchEnd}
        class="transition-all duration-300 ease-out {activeIndex === i
          ? 'scale-125'
          : 'scale-100'}"
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
