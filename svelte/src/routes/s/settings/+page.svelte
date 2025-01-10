<script>
	import Header from '$lib/components/fonts/Header.svelte';
	import { language, translations } from '$lib/stores';
</script>

<svelte:head>
	<title>Settings</title>
</svelte:head>

<Header>{$translations.settings[$language]}</Header>

<div
	class="w-1/2 p-2 items-center justify-center flex flex-col my-8 border-2 py-4 border-sand-900/40 rounded-lg"
>
	<fieldset class="flex flex-col gap-2">
		<legend class="text-xl text-sand-900 mb-3">Select Language</legend>
		<label class="flex items-center space-x-2">
			<input
				type="radio"
				name="language"
				value="en"
				class="appearance-none w-5 h-5 border-2 border-sand-300 rounded-md checked:bg-sand-900 checked:border-sand-900 focus:outline-none focus:ring-2 focus:ring-sand-300 focus:ring-offset-2"
				checked={$language === 'en'}
				onchange={() => {
					language.set('en');
					localStorage.setItem('language', 'en');
				}}
			/>
			<span class="text-sand-900">English</span>
		</label>

		<label class="flex items-center space-x-2">
			<input
				type="radio"
				name="language"
				value="de"
				class="appearance-none w-5 h-5 border-2 border-sand-300 rounded-md checked:bg-sand-900 checked:border-sand-900 focus:outline-none focus:ring-2 focus:ring-sand-300 focus:ring-offset-2"
				checked={$language === 'de'}
				onchange={() => {
					language.set('de');
					localStorage.setItem('language', 'de');
				}}
			/>
			<span class="text-sand-900">Deutsch</span>
		</label>

		<label class="flex items-center space-x-2">
			<input
				type="radio"
				name="language"
				value="ru"
				class="appearance-none w-5 h-5 border-2 border-sand-300 rounded-md checked:bg-sand-900 checked:border-sand-900 focus:outline-none focus:ring-2 focus:ring-sand-300 focus:ring-offset-2"
				checked={$language === 'ru'}
				onchange={() => {
					language.set('ru');
					localStorage.setItem('language', 'ru');
				}}
			/>
			<span class="text-sand-900">Русский</span>
		</label>
	</fieldset>
</div>
