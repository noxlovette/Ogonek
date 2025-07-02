import { env } from "$env/dynamic/private";
import { importSPKI, type JWTPayload, jwtVerify } from "jose";
import logger from "../logger";

const EXPIRY_BUFFER = 30;

export async function ValidateAccess(
  jwt: string | undefined,
): Promise<JWTPayload> {
  const spki = env.JWT_PUBLIC_KEY;
  const alg = env.JWT_ALG || "RS256";

  if (!spki) {
    logger.error("Missing SPKI public key in environment variables");
    throw new Error("Server misconfigured: missing public key");
  }

  if (!jwt) {
    logger.error("ValidateAccess got empty JWT");
    throw new Error("No JWT provided");
  }

  const publicKey = await importSPKI(spki, alg);

  let payload: JWTPayload;

  try {
    const result = await jwtVerify(jwt, publicKey, { algorithms: [alg] });
    payload = result.payload;
  } catch (err) {
    logger.error({ err }, "JWT verification failed");
    throw new Error("Invalid token");
  }

  if (payload.exp && typeof payload.exp === "number") {
    const now = Math.floor(Date.now() / 1000);
    if (payload.exp - now < EXPIRY_BUFFER) {
      logger.warn("Token is about to expire");
      throw new Error("Token about to expire");
    }
  }

  return payload;
}
