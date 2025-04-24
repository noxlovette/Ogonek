<script lang="ts">
  import type { PageData } from "./$types";
  import { Table, H1, UniButton, HeaderEmbellish } from "$lib/components";
  import type { Student, TableConfig } from "$lib/types";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import { PersonStanding } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";

  let { data }: { data: PageData } = $props();

  const { students } = data;

  const studentConfig: TableConfig<Student> = {
    columns: [
      { key: "name", label: "Name" },
      { key: "username", label: "Username" },
      { key: "email", label: "Email" },
      { key: "markdown", label: "Markdown" },
    ],
  };

  let href = "/t/students/st";
</script>

<HeaderEmbellish>
  <H1>Students</H1>
  <form
    method="POST"
    use:enhance={enhanceForm({
      messages: {
        failure: "Failed to generate link",
      },
      handlers: {
        success: async (result) => {
          const link = String(result.data?.link);
          try {
            await navigator.clipboard.writeText(link);
            notification.set({
              message: "Link copied to clipboard!",
              type: "success",
            });
          } catch (err) {
            console.log(err);
            notification.set({ message: "Failed to copy link", type: "error" });
          }
        },
      },
    })}
  >
    <UniButton type="submit" variant="primary" Icon={PersonStanding}
      >Invite Students</UniButton
    >
  </form>
</HeaderEmbellish>
<Table config={studentConfig} {href} items={students} {students} />

<svelte:head>
  <title>Students</title>
</svelte:head>
