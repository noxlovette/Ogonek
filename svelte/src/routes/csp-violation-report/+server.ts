import logger from "$lib/logger.js";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request }) => {
  try {
    const violation = await request.json();

    // Simple structured logging
    logger.warn("🚨 CSP Violation:", {
      blocked: violation["blocked-uri"],
      directive: violation["violated-directive"],
      document: violation["document-uri"],
      timestamp: new Date().toISOString(),
    });

    return json({ status: "received" }, { status: 204 });
  } catch (error) {
    console.error("Invalid CSP violation report:", error);
    return json({ error: "Invalid report" }, { status: 400 });
  }
};
