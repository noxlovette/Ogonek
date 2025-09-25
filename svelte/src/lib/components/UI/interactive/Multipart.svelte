<script lang="ts">
  import { onDestroy } from "svelte";
  import { Ban, Check, Upload, X } from "lucide-svelte";
  import logger from "$lib/logger";
  import { formatFileSize, formatPercentage } from "$lib/utils";
  import { Footnote } from "$lib/components/typography";
  import Caption2 from "$lib/components/typography/Caption2.svelte";
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { m } from "$lib/paraglide/messages";
  import ProgressBar from "./ProgressBar.svelte";
  import { UniButton } from "../forms";
  import { HStack } from "..";

  type UploadStatus = "uploading" | "complete" | "error";

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

  let { taskId = null, folderId = null } = $props();

  let fileUploads: FileUploadState[] = $state([]);

  const CHUNK_SIZE = 5 * 1024 * 1024;

  function calculateChunks(file: File): number {
    return Math.ceil(file.size / CHUNK_SIZE);
  }

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (!input.files?.length) return;

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

    newFiles.forEach(uploadFile);
  }

  async function uploadFile(fileState: FileUploadState) {
    const { id, file } = fileState;
    let uploadIdLocal = "";
    let fileIdLocal = "";
    let s3Key = "";

    try {
      fileState.status = "uploading";
      fileState.abortController = new AbortController();

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

      for (let i = 0; i < parts.length; i++) {
        if (signal.aborted) {
          throw new Error("Upload aborted by user");
        }

        const { partNumber, url } = parts[i];
        const start = (partNumber - 1) * CHUNK_SIZE;
        const end = Math.min(file.size, start + CHUNK_SIZE);
        const chunk = file.slice(start, end);

        try {
          const uploadResult = await uploadWithProgress(
            url,
            chunk,
            signal,
            (loaded) => {
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

          const etag =
            uploadResult.headers.get("ETag")?.replace(/['"]/g, "") || "";
          completedParts.push({ partNumber, etag });

          fileState.progress.uploaded = i + 1;

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
</script>

<HStack>
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
      id="fileInput"
      type="file"
      name="file"
      onchange={handleFileSelect}
      multiple
      class="hidden"
    />
    <Upload class="self-center" />
  </label>

  {#if fileUploads.length > 0}
    <HStack>
      {#each fileUploads as fileState, index (index)}
        <div
          class="ring-default bg-default gap-default flex flex-col rounded-2xl p-2"
        >
          <Footnote>
            {fileState.file.name.length > 15
              ? fileState.file.name.slice(0, 15) + "..."
              : fileState.file.name}
          </Footnote>
          <Caption2>
            {formatFileSize(fileState.file.size)}
          </Caption2>

          <UniButton
            content="Удалить файл"
            Icon={X}
            type="button"
            onclick={() => removeFile(fileState)}
          />

          {#if fileState.status === "uploading"}
            <ProgressBar
              progress={formatPercentage(fileState.progress.percentComplete)}
            />
          {:else if fileState.status === "complete"}
            <Caption1 styling="text-emerald-600"
              >{m.every_sunny_pelican_buzz()}</Caption1
            >
          {:else if fileState.status === "error"}
            <Caption1 styling="text-emerald-600"
              >{m.weird_level_sheep_imagine()}</Caption1
            >
          {/if}
        </div>
      {/each}</HStack
    >
  {/if}
</HStack>
