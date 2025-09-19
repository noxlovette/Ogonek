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
export function isVideoCallUrl(location: string) {
  if (!location || typeof location !== "string") return false;

  const videoCallPatterns = [
    /^https?:\/\/([a-z0-9-]+\.)?zoom\.us\//i,
    /^https?:\/\/meet\.google\.com\//i,
    /^https?:\/\/teams\.microsoft\.com\//i,
    /^https?:\/\/([a-z0-9-]+\.)?webex\.com\//i,
    /^https?:\/\/([a-z0-9-]+\.)?gotomeeting\.com\//i,
    /^https?:\/\/discord\.gg\//i,
    /^https?:\/\/([a-z0-9-]+\.)?discord\.com\//i,
    /^https?:\/\/([a-z0-9-]+\.)?skype\.com\//i,
    /^https?:\/\/([a-z0-9-]+\.)?whereby\.com\//i,
  ];

  return videoCallPatterns.some((pattern) => pattern.test(location.trim()));
}
