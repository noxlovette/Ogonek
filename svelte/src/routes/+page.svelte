<script lang="ts">
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/dashboard' : '/s/dashboard';
</script>

<div class="flex p-8 space-x-6 w-full flex-col items-center">
	<h1 class="xl:text-[4rem] lg:text-[3rem] font-bold leading-[4rem]">Welcome to firelight</h1>
	<div class="mt-8 space-x-2">
		<a
			href="/auth/login"
			class="bg-brick-700 border-milk-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
			>Log In</a
		>
		<a
			{href}
			class="bg-brick-700 border-milk-800 border-2 text-brick-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-brick-100 hover:text-brick-800 transition-colors"
			>Go to Dashboard</a
		>
	</div>
</div>
