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
