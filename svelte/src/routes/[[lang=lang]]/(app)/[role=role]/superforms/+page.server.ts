import { fail } from "@sveltejs/kit";
import { message, superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { z } from "zod/v4";

// Define outside the load function so the adapter can be cached
const schema = z.object({
  name: z.string().default("Hello world!"),
  email: z.email(),
});

export const load = async () => {
  const form = await superValidate(zod4(schema));

  // Always return { form } in load functions
  return { form };
};

export const actions = {
  default: async ({ request }) => {
    const form = await superValidate(request, your_adapter(schema));
    console.log(form);

    if (!form.valid) {
      // Return { form } and things will just work.
      return fail(400, { form });
    }

    // TODO: Do something with the validated form.data

    // Return the form with a status message
    return message(form, "Form posted successfully!");
  },
};
