<script lang="ts">
	import Editor from "$lib/components/Editor.svelte";
	import { setContext } from "svelte";
	import type { PageServerData } from "./$types";

    export let data:PageServerData;

    setContext("article", data.article);

</script>
<Editor />