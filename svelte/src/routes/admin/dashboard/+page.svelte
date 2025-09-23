<script lang="ts">
  import { Toolbar, LargeTitle } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Toolbar>
  <LargeTitle>Admin Dashboard</LargeTitle>
</Toolbar>
You are {data.context.user.role}
