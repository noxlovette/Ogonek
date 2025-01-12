<script lang="ts">
	import { ListTodo } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/tasks' : '/s/tasks';
</script>

<Base href={href}>
	<ListTodo class="xl:size-8 lg:size-7 size-6 mr-2" />
	Tasks
</Base>
