<script lang="ts">
	import Search from '$lib/components/Search.svelte';
	import { user } from '$lib/stores';
	import { Snail } from 'lucide-svelte';
	import {getGreeting} from '$lib/utils';
	import { fade } from 'svelte/transition';
	let hidden = false;
	const greeting = getGreeting();

	console.log($user)
</script>

<div
class="flex flex-col items-center"
>
<h1 class="text-xl lg:text-2xl xl:text-3xl font-serif">Good {greeting}, {$user.first_name}</h1>
</div>
<Search bind:hidden />

