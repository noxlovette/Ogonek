<script lang="ts">
	import { Settings } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/settings' : '/s/settings';
</script>

<Base href={href}>
	<Settings class="xl:size-8 lg:size-7 size-6 mr-2" />

	Settings
</Base>
