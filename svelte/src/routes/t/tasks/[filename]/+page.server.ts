// +page.server.ts
import { env } from "$env/dynamic/private";
import fs from "fs/promises";
import path from "path";

export async function load({ params }) {
    const fullFilePath = decodeURIComponent(params.filename);

    // Extract the filename (sanitize the input)
    const filename = path.basename(fullFilePath);

    // Construct the full path to the file
    const filePath = path.join(env.UPLOAD_DIR, filename);

    try {
        // Read the file content in base64 format
        const base64Content = await fs.readFile(filePath, "utf-8");

        // Convert the base64 content to a Buffer
        const fileBuffer = Buffer.from(base64Content, "base64");

        return {
            filename: filename.split(".")[0], // Original name without extension
            headers: {
                "Content-Type": "application/octet-stream",
                "Content-Disposition": `attachment; filename="${filename}"`,
            },
            body: fileBuffer,
        };
    } catch (error) {
        console.error("Error reading file:", error);
        throw new Error("Failed to load the requested file.");
    }
}
