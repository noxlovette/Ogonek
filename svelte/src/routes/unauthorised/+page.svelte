<!-- Unauthorized.svelte -->
<script>
	import { Ban, DoorClosed, Lock, ShieldAlert, ArrowLeft } from 'lucide-svelte';
</script>

<div class="flex items-center justify-center size-full">
	<div
		class="max-w-2xl w-full bg-white rounded-lg shadow-xl p-8 text-center relative overflow-hidden"
	>
		<!-- Decorative elements -->
		<div class="absolute -top-16 -right-16 transform rotate-45">
			<Ban size={128} class="text-red-600/10" />
		</div>

		<!-- Main content -->
		<div class="relative">
			<div class="flex justify-center mb-6 space-x-4">
				<ShieldAlert class="h-16 w-16 text-red-600 animate-bounce" />
				<Lock class="h-16 w-16 text-milk-400" />
				<DoorClosed class="h-16 w-16 text-red-700" />
			</div>

			<h1 class="text-4xl font-bold text-milk-800 mb-4">Woah there, partner! 🤠</h1>

			<p class="text-xl text-milk-600 mb-6">
				Looks like you're trying to sneak into the VIP section without a backstage pass!
			</p>

			<div class="bg-milk-50 rounded-lg p-4 mb-8">
				<p class="text-milk-500 italic">
					"HTTP 403: The bouncer said no. Maybe try bringing cookies next time? 🍪"
				</p>
			</div>

			<button
				class="inline-flex items-center px-6 py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors duration-200 gap-2 group"
				on:click={() => window.history.back()}
			>
				<ArrowLeft class="group-hover:-tranmilk-x-1 transition-transform duration-200" />
				Return to Safety
			</button>
		</div>
	</div>
</div>
