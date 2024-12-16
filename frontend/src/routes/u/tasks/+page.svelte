<script lang="ts">
	import TaskBar from '$lib/components/TaskBar.svelte';
	import {setContext} from 'svelte';
	import type { PageServerData } from './$types';

	export let data:PageServerData;
	setContext('tasks', data.tasks);

</script>

<div class="flex flex-col items-center justify-center size-full">
<TaskBar />
</div>