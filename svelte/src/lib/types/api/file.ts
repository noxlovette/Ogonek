import type { components } from "./gen/openapi";

export type FileSmall = components["schemas"]["FileSmall"];
export interface InitResponse {
  uploadId: string;
  fileId: string;
  s3Key: string;
  parts: PartUploadUrl[];
}

export interface CompletedPart {
  partNumber: number;
  etag: string;
}

export type UploadStatus = "waiting" | "uploading" | "complete" | "error";

export interface PartUploadUrl {
  partNumber: number;
  url: string;
}

export interface FileUploadState {
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

export interface BatchPresign {
  fileId: string;
  url: string;
}
