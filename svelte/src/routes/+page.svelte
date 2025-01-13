<script lang="ts">
	import { getGreeting } from '$lib/utils';
	const greeting = getGreeting();
</script>

<div class="flex p-8 space-x-6 w-full">
	<div class="flex flex-col">
		<h1 class="xl:text-[4rem] lg:text-[3rem] font-bold leading-[4rem]">
			The digital classroom you've been looking for
		</h1>
		<div class="mt-8 space-x-2">
			<a
				href="/auth/login"
				class="bg-brick-700 border-milk-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
				>Log In</a
			>
			<a
				href="/s/dashboard"
				class="bg-brick-700 border-milk-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
				>Go to Dashboard</a
			>
		</div>
	</div>
	<div class="flex space-x-4">
		<section class="flex flex-col">
			<h2 class="text-4xl font-serif">Teachers</h2>
			<p>Deliver lesson summaries, give homework, and track your students' progress.</p>
		</section>

		<section class="flex flex-col">
			<h2 class="text-4xl font-serif">Students</h2>
			<p>Analyse your performance, visualise your progression, never lose your homework again.</p>
		</section>
	</div>
</div>
