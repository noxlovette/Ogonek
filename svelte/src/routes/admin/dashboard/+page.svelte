<script lang="ts">
  import {
    Title1,
    UniButton,
    VStack,
    HStack,
    Toolbar,
    LargeTitle,
  } from "$lib/components";
  import { FileText, Plus } from "lucide-svelte";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Toolbar>
  <LargeTitle>Admin Dashboard</LargeTitle>
</Toolbar>
You are {data.context.user.role}
