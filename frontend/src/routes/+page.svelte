<script lang="ts">
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

<div class="flex flex-col p-6 border-l-2 border-sand-800/60">
	<h1 class="text-4xl font-bold font-serif">Good {greeting}</h1>
	<div>
		<p>
			You're on <b>Firelight.</b> If you're my student, please log in below. If you aren't... why aren't
			you?
		</p>
		<a href="https://noxlovette.com" class="opacity-80 text-sm"> Made by Danila Volkov </a>
	</div>
	<a
		href="/login"
		class="bg-sand-800/60 text-sand-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-forest-700 transition-colors"
		>Log In</a
	>
</div>
