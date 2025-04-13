<!-- MultipartUploader.svelte -->
<script lang="ts">
  import { onDestroy } from "svelte";
  import UniButton from "../UniButton.svelte";
  import { Ban, Check, Upload, X } from "lucide-svelte";
  import { Label } from "$lib/components/typography";
  import GreySpan from "$lib/components/typography/GreySpan.svelte";

  interface partUploadUrl {
    partNumber: number;
    url: string;
  }

  interface initResponse {
    uploadId: string;
    fileId: string;
    s3Key: string;
    parts: partUploadUrl[];
  }
  let {
    taskId = null,
    folderId = null,
    onComplete = (fileId) => {},
  } = $props();
  // Component state
  let files: File[] = $state([]);
  let uploadProgress: Record<string, { uploaded: number; total: number }> =
    $state({});
  let uploadStatus: Record<
    string,
    "waiting" | "uploading" | "complete" | "error"
  > = $state({});
  let abortControllers: Record<string, AbortController> = {};
  let errorMessages: Record<string, string> = $state({});

  // Constants
  const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB per chunk

  // Handle file selection
  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files) {
      files = [...files, ...Array.from(input.files)];

      // Initialize tracking for each file
      Array.from(input.files).forEach((file) => {
        const fileId = crypto.randomUUID();
        uploadProgress[fileId] = {
          uploaded: 0,
          total: Math.ceil(file.size / CHUNK_SIZE),
        };
        uploadStatus[fileId] = "waiting";
      });

      // Reset input
      input.value = "";
    }
  }

  // Calculate number of chunks needed
  function calculateChunks(file: File): number {
    return Math.ceil(file.size / CHUNK_SIZE);
  }

  // Start upload process for a file
  async function uploadFile(file: File, index: number) {
    const fileId = Object.keys(uploadProgress)[index];
    let uploadIdLocal = "";
    let fileIdLocal = "";
    let s3Key = "";

    try {
      uploadStatus[fileId] = "uploading";

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
        const errorText = await initResponse.text();
        throw new Error(`Failed to initialize upload: ${errorText}`);
      }

      const {
        uploadId,
        fileId: fileDbId,
        s3Key: fileKey,
        parts,
      } = (await initResponse.json()) as initResponse;
      uploadIdLocal = uploadId;
      fileIdLocal = fileDbId;
      s3Key = fileKey;

      // Create abort controller for this upload
      abortControllers[fileId] = new AbortController();
      const signal = abortControllers[fileId].signal;

      const completedParts: { partNumber: number; etag: string }[] = [];

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
          // Upload chunk directly to S3 using presigned URL
          const uploadResult = await fetch(url, {
            method: "PUT",
            body: chunk,
            signal,
          });

          if (!uploadResult.ok) {
            throw new Error(
              `Failed to upload part ${partNumber}: ${uploadResult.statusText}`,
            );
          }

          // Extract ETag from response headers, removing quotes if present
          const etag =
            uploadResult.headers.get("ETag")?.replace(/['"]/g, "") || "";
          completedParts.push({
            partNumber,
            etag,
          });

          // Update progress
          uploadProgress[fileId].uploaded = i + 1;
        } catch (error: any) {
          if (error.name === "AbortError") {
            throw new Error("Upload aborted by user");
          }
          throw new Error(
            `Failed to upload part ${partNumber}: ${error.message}`,
          );
        }
      }

      // 3. Complete the multipart upload
      const completeResponse = await fetch("/api/multipart/complete", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          uploadId,
          fileId,
          s3Key,
          parts: completedParts,
        }),
      });

      if (!completeResponse.ok) {
        const errorText = await completeResponse.text();
        throw new Error(`Failed to complete upload: ${errorText}`);
      }

      uploadStatus[fileId] = "complete";
      onComplete(fileIdLocal);
    } catch (error: any) {
      console.error(`Upload failed for ${file.name}:`, error);
      uploadStatus[fileId] = "error";
      errorMessages[fileId] = error.message || "Upload failed";

      // Try to abort the upload on S3 if it was initialized
      if (uploadIdLocal && fileIdLocal && s3Key) {
        try {
          await fetch("/api/multipart/abort", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              uploadId: uploadIdLocal,
              fileId: fileIdLocal,
              s3Key,
            }),
          });
        } catch (abortError) {
          console.error("Failed to abort upload:", abortError);
        }
      }
    }
  }

  // Start uploading all files
  async function startUploads() {
    files.forEach((file, index) => {
      uploadFile(file, index);
    });
  }

  // Cancel an upload
  function cancelUpload(index: number) {
    const fileId = Object.keys(uploadProgress)[index];
    if (abortControllers[fileId]) {
      abortControllers[fileId].abort();
      uploadStatus[fileId] = "error";
      errorMessages[fileId] = "Upload cancelled by user";
    }
  }

  // Remove a file from the list
  function removeFile(index: number) {
    // Cancel upload if in progress
    cancelUpload(index);

    // Remove from lists
    const fileId = Object.keys(uploadProgress)[index];
    const updatedProgress = { ...uploadProgress };
    const updatedStatus = { ...uploadStatus };
    const updatedErrorMessages = { ...errorMessages };

    delete updatedProgress[fileId];
    delete updatedStatus[fileId];
    delete updatedErrorMessages[fileId];

    uploadProgress = updatedProgress;
    uploadStatus = updatedStatus;
    errorMessages = updatedErrorMessages;

    files = files.filter((_, i) => i !== index);
  }

  // Calculate total size of all files
  const totalSize = $derived(files.reduce((sum, file) => sum + file.size, 0));
  const formattedTotalSize = $derived((totalSize / (1024 * 1024)).toFixed(2));

  // Clean up abort controllers on component destroy
  onDestroy(() => {
    Object.values(abortControllers).forEach((controller) => {
      controller.abort();
    });
  });
</script>

<div class="w-full space-y-4 rounded-lg">
  <label
    for="fileInput"
    class="hover:border-cacao-400 flex h-32 w-full cursor-pointer appearance-none justify-center rounded-md border-2 border-dashed border-stone-300 transition focus:outline-none"
  >
    <span
      class="flex flex-col items-center justify-center space-y-2 text-center"
    >
      <Upload></Upload>
      <Label>Select files for upload</Label>
    </span>
    <input
      id="fileInput"
      type="file"
      class="hidden"
      multiple
      onchange={handleFileSelect}
    />
  </label>

  {#if files.length > 0}
    <div class="space-y-4">
      {#each files as file, i}
        {@const fileId = Object.keys(uploadProgress)[i]}
        {@const progress = uploadProgress[fileId]}
        {@const status = uploadStatus[fileId]}

        <div
          class="flex flex-col space-y-2 rounded-sm border border-stone-200 bg-stone-50 p-2"
        >
          <div class="relative flex items-start justify-between">
            <div>
              <p
                class="max-w-full text-sm font-medium text-stone-700"
                title={file.name}
              >
                {file.name.slice(0, 15)}
              </p>
              <p class="text-xs text-stone-500">
                {(file.size / 1024 / 1024).toFixed(2)} MB
              </p>
            </div>

            <button
              class="absolute top-0 right-0 text-stone-400 hover:text-stone-600"
              onclick={() => removeFile(i)}
              title="Remove file"
              aria-label="Remove file"
            >
              <X />
            </button>
          </div>

          {#if status === "uploading"}
            <div class="mb-1 h-1.5 w-full rounded-full bg-stone-200">
              <div
                class="bg-cacao-600 h-1.5 rounded-full transition-all duration-300"
                style="width: {Math.round(
                  (progress.uploaded / progress.total) * 100,
                )}%"
              ></div>
            </div>
            <p class="text-xs text-stone-500">
              {progress.uploaded} of {progress.total} chunks uploaded ({Math.round(
                (progress.uploaded / progress.total) * 100,
              )}%)
            </p>
          {:else if status === "complete"}
            <div class="flex items-center text-xs text-green-600">
              <Check class="mr-1 size-4" />
              Upload complete
            </div>
          {:else if status === "error"}
            <div class="flex items-center text-xs text-red-600">
              <Ban class="mr-1 size-4"></Ban>
              {errorMessages[fileId] || "Upload failed"}
            </div>
          {/if}
        </div>
      {/each}

      {#if files.some((_f, i) => uploadStatus[Object.keys(uploadStatus)[i]] === "waiting")}
        <UniButton variant="primary" Icon={Upload} onclick={startUploads}>
          Begin
        </UniButton>
      {/if}
    </div>
  {/if}
</div>
