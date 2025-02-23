<script lang="ts">
  import { WholeWord } from "lucide-svelte";
  import Base from "./Base.svelte";
  import {user} from "$lib/stores"

  let href = $user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
</script>

<Base {href}>
  <WholeWord class="size-6 md:mr-2 lg:size-7 xl:size-8" />
  <p class="hidden md:flex">Words</p>
</Base>
