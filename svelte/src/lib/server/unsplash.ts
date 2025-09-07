import { env } from "$env/dynamic/private";
import { createApi } from "unsplash-js";

const serverApi = createApi({
  accessKey: env.UNSPLASH_ACCESS_KEY,
});

export { serverApi as unsplash };
