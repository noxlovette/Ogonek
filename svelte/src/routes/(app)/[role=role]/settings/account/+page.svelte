<script lang="ts">
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";

  import texts from "$lib/texts";
  import { Input } from "$lib/components";

  const { data, form } = $props();
</script>

<svelte:head>
  <title>Настройки</title>
</svelte:head>

<form
  method="POST"
  use:enhance={enhanceForm({
    messages: {
      success: texts.crud.updated,
    },
    shouldUpdate: true,
  })}
  class="flex flex-col gap-4"
  action="?/update"
>
  <Input
    placeholder="Как вас зовут?"
    name="name"
    invalid={form?.name}
    invalidDescription="3+ знаков и никакой херни"
    labelName="Имя"
    value={data.user.name}
  />
  <Input
    type="email"
    placeholder="Email"
    name="email"
    invalid={form?.email}
    verified={data.user.verified}
    invalidDescription="Это не похоже на почту"
    labelName="Электронная почта"
    value={data.user.email}
  />
</form>
