<script lang="ts">
	import Search from '$lib/components/Search.svelte';
	import { user } from '$lib/stores';
	import { Snail } from 'lucide-svelte';

	import { fade } from 'svelte/transition';
	let hidden = false;
</script>

<div
	class:invisible={hidden}
	class="flex flex-col items-center justify-center"
	in:fade={{ duration: 200 }}
>
	<Snail class="size-24 xl:size-32" />
	<h1 class="text-xl lg:text-2xl xl:text-3xl">Welcome, {$user.username}!</h1>
</div>

<Search bind:hidden />
