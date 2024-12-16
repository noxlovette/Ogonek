<script lang="ts">
	import { user } from '$lib/stores';
	import { Snail } from 'lucide-svelte';
</script>

<Snail class="size-32" />
<h1 class="text-4xl">Welcome, {$user.username}!</h1>
<p>Ты должен отсосать мне на коне на сосне при луне</p>