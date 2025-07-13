<script lang="ts">
  import { EmptySpace, H2 } from "$lib/components/typography";
  import { user, teacherData } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { m } from "$lib/paraglide/messages";
  import { Lightbulb } from "lucide-svelte";
  import UniButton from "../UI/UniButton.svelte";
  import { page } from "$app/state";
  import type { TaskSmall } from "$lib/types";
  import { TaskCard } from "$lib/components/cards";

  const { tasks }: { tasks: TaskSmall[] } = $props();
</script>

<div class="flex flex-col gap-4">
  <H2>{m.any_lime_lemur_propel()}</H2>
  {#if tasks.length > 0}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {#each tasks as task (task.id)}
        <TaskCard {task} />
      {/each}
    </div>
  {:else}
    <EmptySpace>
      <form
        method="POST"
        action="/{page.params.role}/tasks?/requestHW"
        class=""
        use:enhance={enhanceForm({
          messages: {
            success: m.minor_mad_hare_buzz(),
          },
        })}
      >
        <input type="hidden" value={$user.username} name="username" />

        <input
          type="hidden"
          value={$teacherData.teacherTelegramId}
          name="teacherTelegramId"
        />
        <UniButton type="submit" variant="primary" Icon={Lightbulb}>
          {m.tense_mealy_kitten_aid()}
        </UniButton>
      </form>
    </EmptySpace>
  {/if}
</div>
