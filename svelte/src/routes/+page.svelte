<script lang="ts">
	import { getGreeting } from '$lib/utils';
	const greeting = getGreeting();
</script>

<h1 class="xl:text-[4rem] lg:text-[3rem] font-bold max-w-5xl p-8 leading-[4rem] my-12">
	The digital classroom you've been looking for
</h1>

<div class="grid grid-cols-2 gap-8 max-w-5xl p-8">
	<section class="flex flex-col">
		<h2 class="text-4xl font-serif">Teachers</h2>
		<p>Manage your classes, create lessons, and track your students' progress all in one place.</p>
	</section>

	<section class="flex flex-col">
		<h2 class="text-4xl font-serif">Students</h2>
		<p>Access your lessons, complete assignments, and track your progress all in one place.</p>
	</section>
</div>
<div class="mt-4 space-x-2">
	<a
		href="/auth/login"
		class="bg-brick-700 border-brick-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
		>Log In</a
	>
	<a
		href="/s/dashboard"
		class="bg-brick-700 border-brick-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
		>Go to Dashboard</a
	>
</div>
