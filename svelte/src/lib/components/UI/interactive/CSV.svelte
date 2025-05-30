<script lang="ts">
  import { Label } from "$lib/components/typography";
  import { notification } from "$lib/stores";
  import type { Card, Deck } from "$lib/types";
  import { UploadCloud, X } from "lucide-svelte";
  import ModalBackGround from "../forms/ModalBackGround.svelte";
  import { extractWordsFromRewordFile } from "$lib/utils";
  import Papa from "papaparse";
  import H2 from "$lib/components/typography/H2.svelte";
  import logger from "$lib/logger";
  interface CSVRow {
    [key: string]: string;
  }

  let {
    deck,
    updatedCards = $bindable([]),
  }: { deck: Deck; updatedCards: Card[] } = $props();
  type importOptions = "csv" | "reword";

  let fileInput: HTMLInputElement | null = $state(null);
  let importOption: importOptions = "reword";
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

  function parseCSV(file: File) {
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

  function importCSV(file: File) {
    Papa.parse(file, {
      header: true,
      delimiter: ";",
      complete: (results) => {
        logger.info("parsing complete", results);
        if (results.data && results.data.length) {
          const csvData = results.data as CSVRow[];

          const newCards = csvData
            .filter((row) => row[importFrontColumn] && row[importBackColumn])
            .map((row) => ({
              id: "",
              deckId: deck.id,
              front: row[importFrontColumn],
              back: row[importBackColumn],
              mediaUrl: undefined,
            }));

          updatedCards = [...updatedCards, ...newCards];

          notification.set({
            type: "success",
            message: `Added ${newCards.length} cards from CSV`,
          });

          closeImportModal();
        } else {
          notification.set({
            type: "error",
            message: "Failed to parse CSV data",
          });
        }
      },
      error: (error) => {
        notification.set({
          type: "error",
          message: `Error importing CSV: ${error.message}`,
        });
      },
    });
  }

  function importFromReword(words: Array<Record<string, any>>) {
    if (!importFrontColumn || !importBackColumn) {
      notification.set({
        type: "error",
        message: "Please select both front and back fields.",
      });
      return;
    }

    const newCards = words
      .filter((word) => word[importFrontColumn] && word[importBackColumn])
      .map((word) => ({
        id: "",
        deckId: deck.id,
        front: word[importFrontColumn],
        back: word[importBackColumn],
        mediaUrl: undefined,
      }));

    logger.debug(newCards);

    updatedCards = [...updatedCards, ...newCards];

    logger.debug(updatedCards);

    notification.set({
      type: "success",
      message: `Added ${newCards.length} cards from .reword`,
    });

    closeImportModal();
  }

  async function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;

    selectedFile = file;

    // Set importOption based on file extension
    const extension = file.name.split(".").pop()?.toLowerCase();
    if (extension === "csv") {
      importOption = "csv";
    } else if (extension === "reword") {
      importOption = "reword";
    } else {
      logger.error("Unsupported file type");
      return;
    }

    if (importOption === "csv") {
      parseCSV(file);
    } else {
      try {
        const words = await extractWordsFromRewordFile(file);
        if (!Array.isArray(words) || words.length === 0) {
          csvPreview = [];
          csvHeaders = [];
          return;
        }

        csvPreview = words;
        csvHeaders = Object.keys(words[0]);

        const frontKeys = ["wrd"];
        const backKeys = [
          "fra",
          "deu",
          "rus",
          "eng",
          "kor",
          "ara",
          "spa",
          "por",
          "ita",
          "ukr",
          "jpn",
          "zhs",
          "zht",
        ];

        importFrontColumn =
          frontKeys.find((key) => csvHeaders.includes(key)) || "wrd";
        importBackColumn =
          backKeys.find((key) => csvHeaders.includes(key)) || "";
      } catch (err) {
        logger.error("Failed to parse .reword file:", err);
      }
    }
  }

  function importCards() {
    if (!importFrontColumn || !importBackColumn) {
      notification.set({
        type: "error",
        message: "Please select columns for front and back of cards",
      });
      return;
    }

    if (!selectedFile) {
      notification.set({
        type: "error",
        message: "No file selected",
      });
      return;
    }

    if (importOption == "csv") {
      importCSV(selectedFile);
    } else {
      importFromReword(csvPreview);
    }
  }
</script>

<ModalBackGround>
  <div class="mb-4 flex min-w-1/2 items-center justify-between">
    <H2>Import Cards</H2>

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
      <p class="text-stone-600 dark:text-stone-400">
        Select either a CSV file with semicolon (;) separated values or a
        .reword Package file
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
          accept=".csv,.reword"
          class="absolute size-full cursor-pointer opacity-0"
          onchange={handleFileSelect}
        />
      </div>
    {:else}
      <div class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <Label>Front</Label>
            <select
              bind:value={importFrontColumn}
              class="w-full rounded-lg border border-stone-300 bg-white p-2 dark:border-stone-700 dark:bg-stone-950"
            >
              <option value="">Select column</option>
              {#each csvHeaders as header, index (index)}
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
              {#each csvHeaders as header, index (index)}
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
                <tbody class="divide-y divide-stone-200 dark:divide-stone-700">
                  {#each csvPreview as row, index (index)}
                    <tr
                      class="even:bg-default odd:bg-white dark:odd:bg-stone-900/30 dark:even:bg-stone-800/50"
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
</ModalBackGround>
