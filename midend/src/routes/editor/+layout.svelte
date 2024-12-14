<script lang="ts">
import { setContext } from "svelte";
import type { LayoutServerData } from "../$types";
import ArticleList from "$lib/components/ArticleList.svelte";

export let data: LayoutServerData;

setContext("meta", data);


</script>
<div class="flex flex-row space-x-8 h-full">
    <div class="w-1/6 bg-floral-200 px-6 py-4 border-r border-floral-300">
      <ArticleList />
    </div>
    <div class="flex flex-1 flex-col  px-6 py-4">
      <slot />
    </div>
  </div>