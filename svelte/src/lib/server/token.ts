import { env } from "$env/dynamic/private";
import logger from "$lib/logger";
import type { Claims } from "$lib/types";
import { importSPKI, jwtVerify } from "jose";

const EXPIRY_BUFFER = 30;

export async function ValidateAccess(jwt: string | undefined): Promise<Claims> {
  const spki = env.JWT_PUBLIC_KEY;
  const alg = env.JWT_ALG || "RS256";

  if (!spki) throw new Error("Server misconfigured: missing public key");
  if (!jwt) throw new Error("No JWT provided");

  const publicKey = await importSPKI(spki, alg);

  try {
    const result = await jwtVerify<Claims>(jwt, publicKey, {
      algorithms: [alg],
    });
    const payload = result.payload;

    if (payload.exp && typeof payload.exp === "number") {
      const now = Math.floor(Date.now() / 1000);
      if (payload.exp - now < EXPIRY_BUFFER) {
        throw new Error("Token about to expire");
      }
    }

    return payload;
  } catch (err: any) {
    logger.error({ err });
    throw new Error("Invalid token");
  }
}
