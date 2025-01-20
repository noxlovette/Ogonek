<script lang="ts">
	import { BookOpenCheck } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/lessons' : '/s/lessons';
</script>

<Base {href}>
	<BookOpenCheck class="xl:size-8 lg:size-7 size-6 md:mr-2" />
	<p class="hidden md:flex">
		Lessons
	</p>
</Base>
