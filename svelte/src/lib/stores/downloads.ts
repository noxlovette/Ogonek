import type { UploadStatus } from "$lib/types";
import { writable } from "svelte/store";

interface UploadProgress {
  [fileId: string]: {
    bytesUploaded: number;
    totalBytes: number;
    status: UploadStatus;
    error?: string;
  };
}

export const uploadProgress = writable<UploadProgress>({});

export const createFileProgress = (fileId: string, totalBytes: number) => {
  uploadProgress.update((progress) => ({
    ...progress,
    [fileId]: {
      bytesUploaded: 0,
      totalBytes,
      status: "waiting" as UploadStatus,
    },
  }));
};

export const updateFileProgress = (fileId: string, bytesUploaded: number) => {
  uploadProgress.update((progress) => ({
    ...progress,
    [fileId]: {
      ...progress[fileId],
      bytesUploaded,
      status: "uploading" as UploadStatus,
    },
  }));
};

export const completeFileUpload = (fileId: string) => {
  uploadProgress.update((progress) => ({
    ...progress,
    [fileId]: {
      ...progress[fileId],
      status: "complete" as UploadStatus,
    },
  }));
};
