<script lang="ts">
	export let task: App.Task;

</script>

<div class="bg-white shadow-md p-8 rounded-lg">
	<h2 class="text-xl font-bold">{task.title}</h2>
	<p class="text-gray-600">{@html task.content}</p>
</div>
