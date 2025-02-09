<script lang="ts">
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/dashboard' : '/s/dashboard';
</script>

<div class="flex p-4 text-center md:text-left md:p-6 lg:p-8 space-x-6 w-full flex-col items-center">
	<h1 class="xl:text-[4rem] text-[3rem] font-bold leading-[3rem] md:leading-[4rem]">
		Welcome to firelight
	</h1>
	<div class="mt-8 space-x-2">
		<a
			href="/auth/login"
			class="bg-cacao-600 dark:bg-milk-900 dark:border-milk-700 dark:hover:bg-milk-600 dark:hover:text-milk-50 md:text-base text-sm border-milk-200 border-2 text-cacao-50 w-20 mt-5 rounded-lg px-3 py-2 md:px-4 md:py-2 text-center hover:bg-cacao-100 hover:text-cacao-800 transition-colors"
			>Log In</a
		>
		<a
			{href}
			class="bg-cacao-600 md:text-base text-sm border-milk-200 dark:border-milk-700 dark:bg-milk-900 dark:hover:bg-milk-600 dark:hover:text-milk-50 border-2 text-cacao-50 w-20 mt-5 rounded-lg px-3 py-2 md:px-4 md:py-2 text-center hover:bg-cacao-100 hover:text-cacao-800 transition-colors"
			>Go to Dashboard</a
		>
	</div>
</div>
