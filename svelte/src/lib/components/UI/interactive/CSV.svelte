<script lang="ts">
  import { Label } from "$lib/components/typography";
  import { notification } from "$lib/stores";
  import type { Card, Deck } from "$lib/types";
  import { UploadCloud, X } from "lucide-svelte";
  import Papa from "papaparse";
  import UniButton from "../UniButton.svelte";
  interface CSVRow {
    [key: string]: string;
  }

  let {
    deck,
    updatedCards = $bindable([]),
  }: { deck: Deck; updatedCards: Card[] } = $props();

  let fileInput: HTMLInputElement | null = $state(null);
  let importFrontColumn = $state("");
  let importBackColumn = $state("");
  let csvHeaders = $state<string[]>([]);
  let csvPreview = $state<any[]>([]);
  let selectedFile: File | null = $state(null);

  function closeImportModal() {
    importFrontColumn = "";
    importBackColumn = "";
    csvHeaders = [];
    csvPreview = [];
    selectedFile = null;
    if (fileInput) fileInput.value = "";
    window.history.back();
  }

  function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;

    // Store the file reference
    selectedFile = file;

    Papa.parse(file, {
      header: true,
      delimiter: ";", // Use semicolon as separator
      preview: 5, // Only preview first 5 rows
      complete: (results) => {
        if (results.data && results.data.length) {
          csvPreview = results.data;
          csvHeaders = results.meta.fields || [];

          // Auto-select columns if they exist
          if (csvHeaders.includes("front")) importFrontColumn = "front";
          if (csvHeaders.includes("back")) importBackColumn = "back";
        } else {
          notification.set({
            type: "error",
            message: "No data found in CSV",
          });
        }
      },
      error: (error) => {
        notification.set({
          type: "error",
          message: `Error parsing CSV: ${error.message}`,
        });
      },
    });
  }

  function importCards() {
    if (!importFrontColumn || !importBackColumn) {
      notification.set({
        type: "error",
        message: "Please select columns for front and back of cards",
      });
      return;
    }

    console.log("starting import");

    // Use the stored file reference instead of reading from input again
    if (!selectedFile) {
      console.error("No file selected");
      notification.set({
        type: "error",
        message: "No file selected",
      });
      return;
    }

    console.log("file found:", selectedFile.name);

    Papa.parse(selectedFile, {
      header: true,
      delimiter: ";",
      complete: (results) => {
        console.log("parsing complete", results);
        if (results.data && results.data.length) {
          const csvData = results.data as CSVRow[];

          console.log("creating new cards from", csvData.length, "rows");
          const newCards = csvData
            .filter((row) => row[importFrontColumn] && row[importBackColumn])
            .map((row) => ({
              id: "",
              deckId: deck.id,
              front: row[importFrontColumn],
              back: row[importBackColumn],
              mediaUrl: undefined,
            }));

          console.log("created", newCards.length, "new cards");

          // Append to existing cards
          updatedCards = [...updatedCards, ...newCards];

          console.log("cards updated, total:", updatedCards.length);
          notification.set({
            type: "success",
            message: `Added ${newCards.length} cards from CSV`,
          });

          closeImportModal();
        } else {
          console.error("No data in results");
          notification.set({
            type: "error",
            message: "Failed to parse CSV data",
          });
        }
      },
      error: (error) => {
        console.error("Error parsing CSV:", error);
        notification.set({
          type: "error",
          message: `Error importing CSV: ${error.message}`,
        });
      },
    });
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-stone-950/50"
>
  <div
    class="w-full max-w-lg rounded-lg bg-white p-6 shadow-xl dark:bg-stone-800"
  >
    <div class="mb-4 flex items-center justify-between">
      <h2 class="text-xl font-bold">Import Cards from CSV</h2>
      <button
        class="text-stone-500 hover:text-stone-700"
        type="button"
        onclick={closeImportModal}
      >
        <X size={20} />
      </button>
    </div>

    <div class="space-y-4">
      {#if !csvHeaders.length}
        <!-- File Upload -->
        <p class="text-stone-600 dark:text-stone-400">
          Select a CSV file with semicolon (;) separated values to import as
          cards.
        </p>

        <div
          class="relative flex h-40 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-stone-300 bg-stone-50 px-4 py-6 dark:border-stone-700 dark:bg-stone-900"
        >
          <UploadCloud class="size-12 opacity-80" />
          <p class="text-stone-600 dark:text-stone-400">
            Drag and drop or click to select
          </p>
          <input
            bind:this={fileInput}
            type="file"
            accept=".csv"
            class="absolute size-full cursor-pointer opacity-0"
            onchange={handleFileSelect}
          />
        </div>
      {:else}
        <!-- Column Mapping -->
        <div class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <Label>Front</Label>
              <select
                bind:value={importFrontColumn}
                class="w-full rounded-lg border border-stone-300 bg-white p-2 dark:border-stone-700 dark:bg-stone-950"
              >
                <option value="">Select column</option>
                {#each csvHeaders as header}
                  <option value={header}>{header}</option>
                {/each}
              </select>
            </div>

            <div>
              <Label>Back</Label>
              <select
                bind:value={importBackColumn}
                class="w-full rounded-lg border border-stone-300 bg-white p-2 dark:border-stone-700 dark:bg-stone-950"
              >
                <option value="">Select column</option>
                {#each csvHeaders as header}
                  <option value={header}>{header}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- Preview -->
          {#if csvPreview.length > 0 && importFrontColumn && importBackColumn}
            <div class="mt-4">
              <h3 class="mb-2 text-sm font-semibold">Preview (first 5 rows)</h3>
              <div
                class="max-h-48 overflow-y-auto rounded-lg border border-stone-300 dark:border-stone-700"
              >
                <table class="w-full">
                  <thead class="bg-stone-100 dark:bg-stone-800">
                    <tr>
                      <th class="p-2 text-left text-sm">Front</th>
                      <th class="p-2 text-left text-sm">Back</th>
                    </tr>
                  </thead>
                  <tbody
                    class="divide-y divide-stone-200 dark:divide-stone-700"
                  >
                    {#each csvPreview as row}
                      <tr
                        class="odd:bg-white even:bg-stone-50 dark:odd:bg-stone-900 dark:even:bg-stone-800/50"
                      >
                        <td class="p-2 text-sm"
                          >{row[importFrontColumn] || "<empty>"}</td
                        >
                        <td class="p-2 text-sm"
                          >{row[importBackColumn] || "<empty>"}</td
                        >
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </div>
          {/if}

          <div class="flex justify-end space-x-2 pt-2">
            <button
              class="rounded-lg border border-stone-300 bg-stone-100 px-4 py-2 font-medium text-stone-700 hover:bg-stone-200 dark:border-stone-700 dark:bg-stone-800 dark:text-stone-300 dark:hover:bg-stone-700"
              onclick={closeImportModal}
            >
              Cancel
            </button>

            <button
              class="bg-cacao-500 hover:bg-cacao-600 rounded-lg px-4 py-2 font-medium text-white disabled:cursor-not-allowed disabled:bg-stone-400"
              disabled={!importFrontColumn || !importBackColumn}
              onclick={importCards}
            >
              Import Cards
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
