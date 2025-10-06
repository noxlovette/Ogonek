<script lang="ts">
  import { Toolbar, LargeTitle } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Toolbar>
  <LargeTitle>Админка</LargeTitle>
</Toolbar>
Вы {data.context.user.role}
