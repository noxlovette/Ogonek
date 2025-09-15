<script lang="ts">
  import { page } from "$app/state";
  import { UniButton } from "$lib/components";
  import { X, Move, Grip, GripVertical } from "lucide-svelte";
  import { panelPosition } from "$lib/stores";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();

  let isDragging = $state(false);
  let dragStart = { x: 0, y: 0 };
  let panelElement: HTMLDivElement;

  function startDrag(event: MouseEvent | TouchEvent) {
    isDragging = true;

    const clientX =
      "touches" in event ? event.touches[0].clientX : event.clientX;
    const clientY =
      "touches" in event ? event.touches[0].clientY : event.clientY;

    dragStart = {
      x: clientX - $panelPosition.x,
      y: clientY - $panelPosition.y,
    };

    // Prevent text selection while dragging
    document.body.style.userSelect = "none";
  }

  function handleDrag(event: MouseEvent | TouchEvent) {
    if (!isDragging) return;

    const clientX =
      "touches" in event ? event.touches[0].clientX : event.clientX;
    const clientY =
      "touches" in event ? event.touches[0].clientY : event.clientY;

    const newX = clientX - dragStart.x;
    const newY = clientY - dragStart.y;

    // Boundary constraints
    const maxX = window.innerWidth - panelElement.offsetWidth;
    const maxY = window.innerHeight - panelElement.offsetHeight;

    const constrainedX = Math.max(0, Math.min(newX, maxX));
    const constrainedY = Math.max(0, Math.min(newY, maxY));

    panelPosition.setPosition({ x: constrainedX, y: constrainedY });
  }

  function stopDrag() {
    isDragging = false;
    document.body.style.userSelect = "";
  }

  // Handle window resize
  function handleResize() {
    const maxX = window.innerWidth - panelElement.offsetWidth;
    const maxY = window.innerHeight - panelElement.offsetHeight;

    if ($panelPosition.x > maxX || $panelPosition.y > maxY) {
      panelPosition.setPosition({
        x: Math.min($panelPosition.x, maxX),
        y: Math.min($panelPosition.y, maxY),
      });
    }
  }
</script>

<svelte:window
  onmousemove={handleDrag}
  onmouseup={stopDrag}
  ontouchmove={handleDrag}
  ontouchend={stopDrag}
  onresize={handleResize}
/>

<div
  bind:this={panelElement}
  class="ring-default fixed z-50 max-w-lg gap-2 rounded-2xl bg-white shadow-2xl
         transition-shadow md:gap-3 lg:gap-4 dark:bg-stone-900
         {isDragging ? 'shadow-3xl cursor-grabbing' : 'shadow-xl'}"
  style="left: {$panelPosition.x}px; top: {$panelPosition.y}px;"
>
  <div
    class="absolute top-4 -left-8 cursor-grab
           px-2 py-1 select-none active:cursor-grabbing"
    onmousedown={startDrag}
    ontouchstart={startDrag}
    role="button"
    tabindex="0"
  >
    <GripVertical></GripVertical>
  </div>
  <UniButton
    href="/{page.params.role}/calendar"
    Icon={X}
    styling="absolute right-1 z-50 top-1"
  />
  <div class="scrollbar-none max-h-[768px] overflow-y-auto p-4 pb-8">
    {@render children()}
  </div>
</div>
