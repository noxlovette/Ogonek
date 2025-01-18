<script lang="ts">
	import { WholeWord } from 'lucide-svelte';
	import External from './External.svelte';
	import { profile } from '$lib/stores';

	let href = $profile.quizletUrl ? $profile.quizletUrl : 'https://quizlet.com';
</script>

<External {href}>
	<WholeWord class="xl:size-8 lg:size-7 size-6 mr-2" />
	Quizlet
</External>
