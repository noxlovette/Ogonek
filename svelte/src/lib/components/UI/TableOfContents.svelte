<!-- TableOfContents.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { H2 } from "../typography";

  // Props with proper TypeScript types
  export let targetSelector: string = ".markdown";
  export let minLevel: number = 1;
  export let maxLevel: number = 6;
  export let title: string = "Contents";

  // Interfaces for better type safety
  interface Heading {
    id: string;
    text: string;
    level: number;
    element: HTMLElement;
  }

  // State with proper typing
  let headings: Heading[] = [];
  let activeId: string = "";
  let observer: IntersectionObserver | null = null;

  // Generate URL-friendly ID from text
  function generateId(text: string | null): string {
    if (!text) return "";
    return text
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "");
  }

  // Scan for headings and build TOC structure
  function buildTOC(): void {
    const container = document.querySelector(targetSelector) as HTMLElement;
    if (!container) {
      console.warn(
        `TableOfContents: Element with selector "${targetSelector}" not found`,
      );
      return;
    }

    const headingSelectors = Array.from(
      { length: maxLevel - minLevel + 1 },
      (_, i) => `h${minLevel + i}`,
    ).join(", ");

    const headingElements = container.querySelectorAll(headingSelectors);

    headings = Array.from(headingElements).map((heading) => {
      const htmlElement = heading as HTMLElement;

      // Generate ID if it doesn't exist
      if (!htmlElement.id) {
        htmlElement.id = generateId(htmlElement.textContent);
      }

      return {
        id: htmlElement.id,
        text: htmlElement.textContent || "",
        level: parseInt(htmlElement.tagName.charAt(1)),
        element: htmlElement,
      };
    });
  }

  // Setup intersection observer for scroll spy
  function setupScrollSpy(): void {
    if (!window.IntersectionObserver || headings.length === 0) return;

    observer = new IntersectionObserver(
      (entries: IntersectionObserverEntry[]) => {
        // Find the first visible heading
        const visibleHeading = entries.find((entry) => entry.isIntersecting);
        if (visibleHeading) {
          activeId = (visibleHeading.target as HTMLElement).id;
        }
      },
      {
        rootMargin: "-20px 0px -60% 0px",
        threshold: 0.1,
      },
    );

    // Observe all headings
    headings.forEach((heading) => {
      observer?.observe(heading.element);
    });
  }

  // Smooth scroll to heading
  function scrollToHeading(id: string, event: Event): void {
    event.preventDefault();
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
      activeId = id;
    }
  }

  // Get Tailwind padding class based on heading level
  function getPaddingClass(level: number): string {
    const minLevelInTOC = Math.min(...headings.map((h) => h.level));
    const relativeLevel = level - minLevelInTOC;

    const paddingClasses = [
      "pl-0", // level 0
      "pl-2", // level 1
      "pl-4", // level 2
      "pl-8", // level 3
      "pl-10", // level 4
      "pl-12", // level 5
    ];

    return (
      paddingClasses[Math.min(relativeLevel, paddingClasses.length - 1)] ||
      "pl-0"
    );
  }

  onMount(() => {
    buildTOC();
    setupScrollSpy();
  });

  onDestroy(() => {
    observer?.disconnect();
  });
</script>

<div
  class="top-4 z-10 h-fit max-h-[calc(100vh-4rem)] overflow-y-auto rounded-lg px-3 backdrop-blur-sm md:sticky"
>
  {#if title}
    <div class="mb-3">
      <H2>
        {title}
      </H2>
    </div>
  {/if}

  {#if headings.length > 0}
    <nav class="space-y-1" aria-label="Table of contents">
      <ul class="list-none space-y-1">
        {#each headings as heading (heading.id)}
          <li class={getPaddingClass(heading.level)}>
            <a
              href="#{heading.id}"
              class="block px-2 py-1.5 text-sm text-stone-700 transition-colors hover:bg-stone-100 hover:text-stone-900 dark:text-stone-300 dark:hover:bg-stone-700 dark:hover:text-stone-100
                  {activeId === heading.id
                ? 'border-cacao-500 bg-cacao-50 text-cacao-700 dark:border-cacao-400 dark:bg-cacao-900/20 dark:text-cacao-100 border-l-2 font-medium'
                : ''}"
              on:click={(e) => scrollToHeading(heading.id, e)}
            >
              {heading.text}
            </a>
          </li>
        {/each}
      </ul>
    </nav>
  {:else}
    <p class="text-sm text-stone-500 dark:text-stone-400">
      No headings found in {targetSelector}
    </p>
  {/if}
</div>
