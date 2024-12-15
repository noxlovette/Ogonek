<script lang="ts">
	import { user } from '$lib/stores';
</script>

<h1>Welcome, {$user.username}!</h1>
<p>Here's your Quizlet URL: <a href="{$user.quizlet_url}">{$user.quizlet_url}</a></p>