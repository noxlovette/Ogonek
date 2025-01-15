<script lang="ts">


	let { children }= $props();
</script>


<div class="flex flex-col justify-center items-center overflow-auto flex-1 size-full">
		{@render children?.()}
</div>
