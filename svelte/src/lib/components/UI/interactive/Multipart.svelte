<script lang="ts">
  import { onDestroy } from "svelte";
  import UniButton from "../forms/buttons/UniButton.svelte";
  import { Ban, Check, Upload, X } from "lucide-svelte";
  import { Caption1 } from "$lib/components/typography";
  import logger from "$lib/logger";

  type UploadStatus = "waiting" | "uploading" | "complete" | "error";

  interface PartUploadUrl {
    partNumber: number;
    url: string;
  }

  interface InitResponse {
    uploadId: string;
    fileId: string;
    s3Key: string;
    parts: PartUploadUrl[];
  }

  interface CompletedPart {
    partNumber: number;
    etag: string;
  }

  interface FileUploadState {
    id: string;
    file: File;
    progress: {
      uploaded: number;
      total: number;
      bytes: number;
      totalBytes: number;
      percentComplete: number;
    };
    status: UploadStatus;
    errorMessage?: string;
    abortController?: AbortController;
  }

  let { taskId = null, folderId = null, onComplete = () => {} } = $props();

  let fileUploads: FileUploadState[] = $state([]);

  const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB per chunk

  function calculateChunks(file: File): number {
    return Math.ceil(file.size / CHUNK_SIZE);
  }

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (!input.files?.length) return;

    // Add new files to the uploads array
    const newFiles = Array.from(input.files).map((file) => ({
      id: crypto.randomUUID(),
      file,
      progress: {
        uploaded: 0,
        total: calculateChunks(file),
        bytes: 0,
        totalBytes: file.size,
        percentComplete: 0,
      },
      status: "waiting" as UploadStatus,
    }));

    fileUploads = [...fileUploads, ...newFiles];

    input.value = "";
  }

  // Start upload process for a file
  async function uploadFile(fileState: FileUploadState) {
    const { id, file } = fileState;
    let uploadIdLocal = "";
    let fileIdLocal = "";
    let s3Key = "";

    try {
      fileState.status = "uploading";
      fileState.abortController = new AbortController();

      // 1. Initialize multipart upload
      const initResponse = await fetch("/api/multipart/init", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          fileName: file.name,
          contentType: file.type || "application/octet-stream",
          fileSize: file.size,
          totalParts: calculateChunks(file),
          parentId: folderId,
          taskId: taskId,
        }),
      });

      if (!initResponse.ok) {
        throw new Error(
          `Failed to initialize upload: ${await initResponse.text()}`,
        );
      }

      const {
        uploadId,
        fileId,
        s3Key: fileKey,
        parts,
      } = (await initResponse.json()) as InitResponse;
      uploadIdLocal = uploadId;
      fileIdLocal = fileId;
      s3Key = fileKey;

      const signal = fileState.abortController.signal;
      const completedParts: CompletedPart[] = [];

      // 2. Upload each chunk directly to S3
      for (let i = 0; i < parts.length; i++) {
        // Check if upload was aborted
        if (signal.aborted) {
          throw new Error("Upload aborted by user");
        }

        const { partNumber, url } = parts[i];
        const start = (partNumber - 1) * CHUNK_SIZE;
        const end = Math.min(file.size, start + CHUNK_SIZE);
        const chunk = file.slice(start, end);

        try {
          // Upload chunk directly to S3 using presigned URL with progress tracking
          const uploadResult = await uploadWithProgress(
            url,
            chunk,
            signal,
            (loaded) => {
              // Update byte-level progress
              const chunkStart = (partNumber - 1) * CHUNK_SIZE;
              fileState.progress.bytes = chunkStart + loaded;
              fileState.progress.percentComplete =
                (fileState.progress.bytes / fileState.progress.totalBytes) *
                100;
            },
          );

          if (!uploadResult.ok) {
            throw new Error(
              `Failed to upload part ${partNumber}: ${uploadResult.statusText}`,
            );
          }

          // Extract ETag from response headers, removing quotes if present
          const etag =
            uploadResult.headers.get("ETag")?.replace(/['"]/g, "") || "";
          completedParts.push({ partNumber, etag });

          // Update chunk-level progress
          fileState.progress.uploaded = i + 1;

          // Ensure bytes progress is accurate at chunk boundaries
          const chunkEnd = Math.min(file.size, partNumber * CHUNK_SIZE);
          fileState.progress.bytes = chunkEnd;
          fileState.progress.percentComplete =
            (chunkEnd / fileState.progress.totalBytes) * 100;
        } catch (error) {
          console.error(error);
          throw new Error("Failed to upload");
        }
      }

      // 3. Complete the multipart upload
      const completeResponse = await fetch(
        `/api/multipart/complete?taskId=${taskId}`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            uploadId,
            fileId: fileIdLocal,
            s3Key,
            parts: completedParts,
          }),
        },
      );

      if (!completeResponse.ok) {
        throw new Error(
          `Failed to complete upload: ${await completeResponse.text()}`,
        );
      }

      fileState.status = "complete";
      onComplete(fileIdLocal);
    } catch (error: any) {
      logger.error(`Upload failed for ${file.name}:`, error);
      fileState.status = "error";
      fileState.errorMessage = "Upload failed";

      // Try to abort the upload on S3 if it was initialized
      if (uploadIdLocal && s3Key) {
        try {
          await fetch("/api/multipart/abort", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              uploadId: uploadIdLocal,
              fileId: id,
              s3Key,
            }),
          });
        } catch (abortError: any) {
          logger.error("Failed to abort upload:", abortError);
        }
      }
    }
  }

  // Start uploading all waiting files
  function startUploads() {
    fileUploads
      .filter((upload) => upload.status === "waiting")
      .forEach(uploadFile);
  }

  // Cancel an upload
  function cancelUpload(fileState: FileUploadState) {
    fileState.abortController?.abort();
    fileState.status = "error";
    fileState.errorMessage = "Upload cancelled by user";
  }

  // Remove a file from the list
  function removeFile(fileState: FileUploadState) {
    // Cancel upload if in progress
    if (fileState.status === "uploading") {
      cancelUpload(fileState);
    }

    // Remove from list
    fileUploads = fileUploads.filter((upload) => upload.id !== fileState.id);
  }

  // Clean up abort controllers on component destroy
  onDestroy(() => {
    fileUploads.forEach((upload) => {
      upload.abortController?.abort();
    });
  });

  // Upload with progress tracking
  async function uploadWithProgress(
    url: string,
    data: Blob,
    signal: AbortSignal,
    onProgress: (loaded: number) => void,
  ): Promise<Response> {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();

      xhr.upload.addEventListener("progress", (event) => {
        if (event.lengthComputable) {
          onProgress(event.loaded);
        }
      });

      xhr.addEventListener("loadend", () => {
        if (xhr.readyState === 4) {
          if (xhr.status >= 200 && xhr.status < 300) {
            // Convert XHR response to fetch Response for consistency
            const response = new Response(xhr.response, {
              status: xhr.status,
              statusText: xhr.statusText,
              headers: new Headers({
                ETag: xhr.getResponseHeader("ETag") || "",
              }),
            });
            resolve(response);
          } else {
            reject(new Error(`HTTP error ${xhr.status}: ${xhr.statusText}`));
          }
        }
      });

      xhr.addEventListener("error", () => {
        reject(new Error("Network error occurred"));
      });

      xhr.addEventListener("abort", () => {
        reject(new Error("Upload aborted"));
      });

      // Initialize the request
      xhr.open("PUT", url);
      xhr.send(data);

      // Handle abort signal
      if (signal) {
        signal.addEventListener("abort", () => {
          xhr.abort();
        });
      }
    });
  }

  // Format file size with appropriate units
  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " KB";
    return (bytes / (1024 * 1024)).toFixed(2) + " MB";
  }

  // Format percentage display
  function formatPercentage(value: number): string {
    return Math.min(100, Math.max(0, Math.round(value))) + "%";
  }

  let isDragging = $state(false);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (!e.dataTransfer?.files.length) return;

    // Create a synthetic event that mimics an input change event
    const mockEvent = {
      target: {
        files: e.dataTransfer.files,
      },
    } as unknown as Event;

    // Use your existing file handling function
    handleFileSelect(mockEvent);
  }
  let fileInput: HTMLInputElement;
</script>

<div class="flex flex-col space-y-2 rounded-lg">
  <label
    for="fileInput"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    aria-label="File upload area"
    class="relative flex flex-1 cursor-pointer
			 flex-col items-center justify-center rounded-lg border-2
			 border-dashed p-12 duration-200
			 {isDragging
      ? 'border-accent bg-accent'
      : 'border-stone-300/30 hover:border-stone-400 dark:border-stone-600/30 dark:bg-stone-950 dark:hover:border-stone-700'}"
  >
    <input
      bind:this={fileInput}
      id="fileInput"
      type="file"
      name="file"
      onchange={handleFileSelect}
      multiple
      class="hidden"
    />
    <span
      class="flex flex-col items-center justify-center space-y-2 text-center"
    >
      <Upload />
    </span>
  </label>

  {#if fileUploads.length > 0}
    <div class="space-y-4">
      {#each fileUploads as fileState, index (index)}
        <div
          class="ring-default bg-default flex flex-col space-y-2 rounded-sm p-2"
        >
          <div class="relative flex items-start justify-between">
            <div>
              <p
                class="max-w-full text-sm font-medium text-stone-700 dark:text-stone-400"
                title={fileState.file.name}
              >
                {fileState.file.name.length > 15
                  ? fileState.file.name.slice(0, 15) + "..."
                  : fileState.file.name}
              </p>
              <p class="text-xs text-stone-500">
                {formatFileSize(fileState.file.size)}
              </p>
            </div>

            <button
              class="absolute top-0 right-0 text-stone-400 hover:text-stone-600"
              onclick={() => removeFile(fileState)}
              title="Remove file"
              aria-label="Remove file"
            >
              <X />
            </button>
          </div>

          {#if fileState.status === "uploading"}
            <div
              class="mb-1 h-1.5 w-full rounded-full bg-stone-200 dark:bg-stone-600"
            >
              <div
                class="bg-accent h-1.5 rounded-full duration-150 dark:bg-stone-100"
                style="width: {formatPercentage(
                  fileState.progress.percentComplete,
                )}"
              ></div>
            </div>
            <p class="text-xs text-stone-500">
              {formatFileSize(fileState.progress.bytes)} of {formatFileSize(
                fileState.progress.totalBytes,
              )}
              ({formatPercentage(fileState.progress.percentComplete)})
            </p>
          {:else if fileState.status === "complete"}
            <div class="flex items-center text-xs text-green-600">
              <Check class="mr-1 size-4" />
              Upload complete
            </div>
          {:else if fileState.status === "error"}
            <div class="flex items-center text-xs text-red-600">
              <Ban class="mr-1 size-4" />
              {fileState.errorMessage || "Upload failed"}
            </div>
          {/if}
        </div>
      {/each}

      {#if fileUploads.some((upload) => upload.status === "waiting")}
        <UniButton variant="primary" Icon={Upload} onclick={startUploads}>
          Begin Upload
        </UniButton>
      {/if}
    </div>
  {/if}
</div>
