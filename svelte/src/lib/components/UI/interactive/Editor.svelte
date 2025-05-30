<script lang="ts">
  import { parseMarkdown } from "@noxlovette/svarog";

  let {
    markdownContent = $bindable(
      "# Start writing\n\nYour **markdown** goes here...",
    ),
  } = $props();

  let htmlContent = $state("");
  let preview = $state(false);
  let textareaRef = $state(HTMLTextAreaElement.prototype);

  // Undo/Redo functionality
  let history: string[] = $state([markdownContent]);
  let historyIndex = $state(0);
  let lastSaveTime = $state(Date.now());

  async function updatePreview(content: string) {
    htmlContent = await parseMarkdown(content);
  }

  $effect(() => {
    updatePreview(markdownContent);

    // Save to history after a delay to group rapid changes
    const now = Date.now();
    if (now - lastSaveTime > 500) {
      // 500ms delay
      saveToHistory();
      lastSaveTime = now;
    }
  });

  function saveToHistory() {
    // Don't save if content hasn't changed
    if (history[historyIndex] === markdownContent) return;

    // Remove any future history if we're not at the end
    history = history.slice(0, historyIndex + 1);

    // Add new state
    history.push(markdownContent);
    historyIndex = history.length - 1;

    // Limit history size
    if (history.length > 50) {
      history = history.slice(1);
      historyIndex = history.length - 1;
    }
  }

  function undo() {
    if (historyIndex > 0) {
      historyIndex--;
      markdownContent = history[historyIndex];
    }
  }

  function redo() {
    if (historyIndex < history.length - 1) {
      historyIndex++;
      markdownContent = history[historyIndex];
    }
  }

  // Toolbar functions
  function insertMarkdown(
    before: string,
    after: string = "",
    placeholder: string = "text",
  ) {
    if (!textareaRef) return;

    const start = textareaRef.selectionStart;
    const end = textareaRef.selectionEnd;
    const selectedText = markdownContent.substring(start, end);
    const replacement = selectedText || placeholder;

    const newText =
      markdownContent.substring(0, start) +
      before +
      replacement +
      after +
      markdownContent.substring(end);

    markdownContent = newText;

    // Set cursor position
    setTimeout(() => {
      const newStart = start + before.length;
      const newEnd = newStart + replacement.length;
      textareaRef.setSelectionRange(newStart, newEnd);
      textareaRef.focus();
    }, 0);
  }

  function insertLink() {
    if (!textareaRef) return;

    const start = textareaRef.selectionStart;
    const end = textareaRef.selectionEnd;
    const selectedText = markdownContent.substring(start, end);

    const linkText = selectedText || "link text";
    const replacement = `[${linkText}](url)`;

    const newText =
      markdownContent.substring(0, start) +
      replacement +
      markdownContent.substring(end);

    markdownContent = newText;

    // Select the URL part
    setTimeout(() => {
      const urlStart = start + linkText.length + 3; // After "[linkText]("
      const urlEnd = urlStart + 3; // "url"
      textareaRef.setSelectionRange(urlStart, urlEnd);
      textareaRef.focus();
    }, 0);
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.ctrlKey || event.metaKey) {
      switch (event.key) {
        case "b":
          event.preventDefault();
          insertMarkdown("**", "**", "bold text");
          break;
        case "i":
          event.preventDefault();
          insertMarkdown("*", "*", "italic text");
          break;
        case "k":
          event.preventDefault();
          insertLink();
          break;
        case "z":
          event.preventDefault();
          if (event.shiftKey) {
            redo();
          } else {
            undo();
          }
          break;
        case "y":
          event.preventDefault();
          redo();
          break;
      }
    }
    if (event.key === "Enter") {
      const start = textareaRef.selectionStart;
      const lines = markdownContent.substring(0, start).split("\n");
      const currentLine = lines[lines.length - 1];

      // Check if current line is a list item
      const listMatch = currentLine.match(/^(\s*)([-*+]|\d+\.)\s/);
      if (listMatch) {
        event.preventDefault();
        const indent = listMatch[1];
        const listMarker = listMatch[2];

        // If the current line only contains the list marker (empty item), remove it
        if (currentLine.trim() === listMarker) {
          // Remove the empty list item
          const lineStart = start - currentLine.length;
          markdownContent =
            markdownContent.substring(0, lineStart) +
            markdownContent.substring(start);
          setTimeout(() => {
            textareaRef.setSelectionRange(lineStart, lineStart);
          }, 0);
          return;
        }

        // Add new list item
        let newMarker = listMarker;
        if (/^\d+\./.test(listMarker)) {
          // For numbered lists, increment the number
          const num = parseInt(listMarker) + 1;
          newMarker = `${num}.`;
        }

        const newListItem = `\n${indent}${newMarker} `;
        const newText =
          markdownContent.substring(0, start) +
          newListItem +
          markdownContent.substring(start);

        markdownContent = newText;

        setTimeout(() => {
          const newPos = start + newListItem.length;
          textareaRef.setSelectionRange(newPos, newPos);
        }, 0);
      }
    }

    // Handle Tab for list indentation
    if (event.key === "Tab") {
      const start = textareaRef.selectionStart;
      const lines = markdownContent.substring(0, start).split("\n");
      const currentLine = lines[lines.length - 1];

      if (currentLine.match(/^\s*([-*+]|\d+\.)\s/)) {
        event.preventDefault();
        const lineStart = start - currentLine.length;
        const indent = event.shiftKey
          ? currentLine.replace(/^ {2}/, "") // Remove 2 spaces for Shift+Tab
          : "  " + currentLine; // Add 2 spaces for Tab

        markdownContent =
          markdownContent.substring(0, lineStart) +
          indent +
          markdownContent.substring(start);

        setTimeout(() => {
          const offset = event.shiftKey ? -2 : 2;
          textareaRef.setSelectionRange(start + offset, start + offset);
        }, 0);
      }
    }
  }
</script>

<div class="col-span-2 flex size-full flex-col gap-4">
  <div id="header" class="flex items-center space-x-4">
    <h1 class="text-2xl font-bold">Markdown</h1>
    <button
      onclick={() => (preview = false)}
      class="hover:bg-cacao-200 rounded-lg px-2 py-1 text-sm transition-colors"
      class:chosen={!preview}>Editor</button
    >
    <button
      onclick={() => (preview = true)}
      class="hover:bg-cacao-200 rounded-lg px-2 py-1 text-sm transition-colors"
      class:chosen={preview}>Preview</button
    >
  </div>

  {#if !preview}
    <!-- Toolbar -->
    <div
      class="flex flex-wrap gap-2 rounded-lg border border-stone-300 bg-stone-50 p-2 dark:border-stone-700 dark:bg-stone-900"
    >
      <button
        onclick={() => insertMarkdown("**", "**", "bold text")}
        class="rounded px-3 py-1 text-sm font-bold transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Bold (Ctrl+B)"
      >
        B
      </button>

      <button
        onclick={() => insertMarkdown("*", "*", "italic text")}
        class="rounded px-3 py-1 text-sm italic transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Italic (Ctrl+I)"
      >
        I
      </button>

      <button
        onclick={() => insertMarkdown("`", "`", "code")}
        class="rounded px-3 py-1 font-mono text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Inline Code"
      >
        &lt;/&gt;
      </button>

      <div class="w-px bg-stone-300 dark:bg-stone-600"></div>

      <button
        onclick={() => insertMarkdown("# ", "", "Heading")}
        class="rounded px-3 py-1 text-sm font-bold transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Heading 1"
      >
        H1
      </button>

      <button
        onclick={() => insertMarkdown("## ", "", "Heading")}
        class="rounded px-3 py-1 text-sm font-bold transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Heading 2"
      >
        H2
      </button>

      <button
        onclick={() => insertMarkdown("### ", "", "Heading")}
        class="rounded px-3 py-1 text-sm font-bold transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Heading 3"
      >
        H3
      </button>

      <div class="w-px bg-stone-300 dark:bg-stone-600"></div>

      <button
        onclick={() => insertMarkdown("- ", "", "List item")}
        class="rounded px-3 py-1 text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Bullet List"
      >
        • List
      </button>

      <button
        onclick={() => insertMarkdown("1. ", "", "List item")}
        class="rounded px-3 py-1 text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Numbered List"
      >
        1. List
      </button>

      <button
        onclick={() => insertMarkdown("> ", "", "Quote")}
        class="rounded px-3 py-1 text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Quote"
      >
        Quote
      </button>

      <div class="w-px bg-stone-300 dark:bg-stone-600"></div>

      <button
        onclick={insertLink}
        class="rounded px-3 py-1 text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Link (Ctrl+K)"
      >
        🔗 Link
      </button>

      <button
        onclick={() => insertMarkdown("```\n", "\n```", "code block")}
        class="rounded px-3 py-1 font-mono text-sm transition-colors hover:bg-stone-200 dark:hover:bg-stone-700"
        title="Code Block"
      >
        Code
      </button>
    </div>
  {/if}

  <div class="flex flex-1">
    {#if !preview}
      <!-- Editor -->
      <div class="flex w-full flex-col">
        <textarea
          bind:this={textareaRef}
          bind:value={markdownContent}
          onkeydown={handleKeyDown}
          class="
          focus:border-cacao-500 focus:ring-cacao-500/20 min-h-[400px] w-full resize-none rounded-2xl border border-stone-300 bg-white px-4 py-2 font-mono text-base leading-relaxed text-stone-900 placeholder-stone-400 shadow-sm transition-all focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-700 dark:bg-stone-950 dark:text-stone-100
          "
          spellcheck="false"
          placeholder="Start typing your markdown here..."
        ></textarea>
      </div>
    {:else}
      <div
        class="markdown prose prose-stone dark:prose-invert w-full max-w-none rounded-lg p-4 shadow-sm dark:border-stone-900"
      >
        {@html htmlContent}
      </div>
    {/if}
  </div>
</div>
