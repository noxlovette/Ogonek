<script lang="ts">
	import { formatDateTime, getAvatar } from '$lib/utils';
	import type { Student } from '$lib/types';
	import Card from './Card.svelte';
	import { user } from '$lib/stores';
	interface Props {
		student: Student;
	}

	let { student }: Props = $props();

	const avatarURL = getAvatar(student.id);
</script>

<Card>
	<div class="flex items-center space-x-2">
		<img src={avatarURL} alt="avatar" class="w-10 h-10 rounded-full" />
		<h3>{student.name}</h3>
	</div>
	<p>{student.email}</p>
</Card>
