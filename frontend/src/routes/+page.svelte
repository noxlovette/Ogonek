<script lang="ts">
	import TaskBar from '$lib/components/TaskBar.svelte';

	function getGreeting() {
		const date = new Date();
		const hours = date.getHours();

		if (hours >= 5 && hours < 12) {
			return 'morning';
		} else if (hours >= 12 && hours < 18) {
			return 'afternoon';
		} else if (hours >= 18 && hours < 22) {
			return 'evening';
		} else {
			return 'night';
		}
	}

	const greeting = getGreeting();
</script>

<h1 class="text-3xl font-bold">Good {greeting}</h1>
