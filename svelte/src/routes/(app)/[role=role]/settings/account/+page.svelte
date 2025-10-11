<script lang="ts">
  import { Input, Title1 } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data, form }: PageProps = $props();
</script>

<Title1>Настройки аккаунта</Title1>

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
