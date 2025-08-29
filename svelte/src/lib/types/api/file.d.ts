export interface FileSmall {
  id: string;
  name: string;
  s3Key: string;
  mimeType?: string;
  size: number;
  ownerId: string;
}

export interface File {
  id: string;
  name: string;
  s3Key: string;
  mimeType?: string;
  size: number;
  ownerId: string;
}
