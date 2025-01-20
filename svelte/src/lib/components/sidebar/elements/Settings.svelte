<script lang="ts">
	import { Settings } from 'lucide-svelte';
	import Base from './Base.svelte';
	import { user } from '$lib/stores';

	let href = $user.role === 'teacher' ? '/t/settings' : '/s/settings';
</script>

<Base {href}>
	<Settings class="xl:size-8 lg:size-7 size-6 md:mr-2" />
	<p class="hidden md:flex">Settings</p>
</Base>
