import { strFromU8, unzipSync } from "fflate";

export async function extractWordsFromRewordFile(file: File) {
  const buffer = await file.arrayBuffer();
  const uint8 = new Uint8Array(buffer);

  const files = unzipSync(uint8);

  // Assume only one file inside the archive — get the first one
  const [filename, content] = Object.entries(files)[0] || [];

  if (!filename || !content) {
    throw new Error("No file found in .reword archive.");
  }

  const jsonText = strFromU8(content);
  const data = JSON.parse(jsonText);

  if (Array.isArray(data)) return data;
  if (Array.isArray(data.words)) return data.words;

  throw new Error("Invalid JSON format in .reword file");
}
interface VideoCallService {
  name: string;
  pattern: RegExp;
}

const VIDEO_CALL_SERVICES: VideoCallService[] = [
  { name: "Zoom", pattern: /^https?:\/\/([a-z0-9-]+\.)?zoom\.us\//i },
  { name: "Google Meet", pattern: /^https?:\/\/meet\.google\.com\//i },
  { name: "Microsoft Teams", pattern: /^https?:\/\/teams\.microsoft\.com\//i },
  { name: "Webex", pattern: /^https?:\/\/([a-z0-9-]+\.)?webex\.com\//i },
  {
    name: "GoToMeeting",
    pattern: /^https?:\/\/([a-z0-9-]+\.)?gotomeeting\.com\//i,
  },
  {
    name: "Discord",
    pattern: /^https?:\/\/(discord\.gg|([a-z0-9-]+\.)?discord\.com)\//i,
  },
  { name: "Skype", pattern: /^https?:\/\/([a-z0-9-]+\.)?skype\.com\//i },
  { name: "Whereby", pattern: /^https?:\/\/([a-z0-9-]+\.)?whereby\.com\//i },
];

export function getVideoCallService(location: string): string | null {
  if (!location || typeof location !== "string") return null;

  const trimmedLocation = location.trim();
  const service = VIDEO_CALL_SERVICES.find((service) =>
    service.pattern.test(trimmedLocation),
  );

  return service?.name ?? null;
}

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " KB";
  return (bytes / (1024 * 1024)).toFixed(2) + " MB";
}

export function formatPercentage(value: number): string {
  return Math.min(100, Math.max(0, Math.round(value))) + "%";
}
