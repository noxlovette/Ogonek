<script lang="ts">
	import { Bookmark } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/lessons' : '/s/lessons';
</script>

<Base {href}>
	<Bookmark class="xl:size-8 lg:size-7 size-6 mr-2" />
	Lessons
</Base>
