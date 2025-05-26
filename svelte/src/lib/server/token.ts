import { env } from "$env/dynamic/private";
import { importSPKI, type JWTPayload, jwtVerify } from "jose";
import logger from "../logger";

const EXPIRY_BUFFER = 30; // seconds

export async function ValidateAccess(jwt: string): Promise<JWTPayload> {
  const spki = env.JWT_PUBLIC_KEY;
  const alg = env.JWT_ALG || "RS256";

  if (!spki) {
    logger.error("Missing SPKI public key in environment variables");
    throw new Error("Server misconfigured: missing public key");
  }

  const publicKey = await importSPKI(spki, alg);

  logger.info("Verifying access token");
  let payload: JWTPayload;

  try {
    const result = await jwtVerify(jwt, publicKey, { algorithms: [alg] });
    payload = result.payload;
  } catch (err) {
    logger.error("JWT verification failed", err);
    throw new Error("Invalid token");
  }
  logger.debug({ payload }, "Token payload");
  logger.info("Access verification complete");

  if (payload.exp && typeof payload.exp === "number") {
    const now = Math.floor(Date.now() / 1000);
    if (payload.exp - now < EXPIRY_BUFFER) {
      logger.warn("Token is about to expire");
      throw new Error("Token about to expire");
    }
  }

  return payload;
}
