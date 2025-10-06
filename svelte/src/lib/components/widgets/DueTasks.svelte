<script lang="ts">
  import { EmptySpace, Title2 } from "$lib/components/typography";
  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { Lightbulb } from "@lucide/svelte";
  import UniButton from "../UI/forms/buttons/UniButton.svelte";
  import { page } from "$app/state";
  import type { TaskSmall } from "$lib/types";
  import { TaskCard } from "$lib/components/cards";
  import { HStack } from "../UI";
  import texts from "$lib/texts";

  const { tasks }: { tasks: TaskSmall[] } = $props();
</script>

<HStack>
  <Title2>Задания</Title2>
  {#each tasks as task (task.id)}
    <div class="grid w-full grid-cols-1 items-stretch gap-4">
      <TaskCard {task} />
    </div>
  {:else}
    <EmptySpace>
      <form
        method="POST"
        action="/{page.params.role}/tasks?/requestHW"
        class=""
        use:enhance={enhanceForm({
          messages: {
            success: texts.notifications.homeworkRequested,
          },
        })}
      >
        <UniButton
          content="Попросить ДЗ"
          type="submit"
          variant="primary"
          Icon={Lightbulb}
        ></UniButton>
      </form>
    </EmptySpace>
  {/each}
</HStack>
