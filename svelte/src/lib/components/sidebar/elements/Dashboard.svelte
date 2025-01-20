<script lang="ts">
	import { House } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/dashboard' : '/s/dashboard';
</script>

<Base {href}>
	<House class="xl:size-8 lg:size-7 size-6 md:mr-2 " />
	<p class="hidden md:flex">
		Dashboard
	</p>
</Base>
