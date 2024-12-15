<script lang="ts">
	import { AudioLines, Book, BookCopy } from "lucide-svelte";


</script>

<div class="flex flex-col border-2 border-sand-900/20 p-4 rounded-lg">
    <h2 class="text-xl">
        Useful Links
    </h2>
    <a href="https://forvo.com/languages/en/"
    
    class="px-2 py-4 inline-flex">
				<AudioLines class="size-8 mr-2 " />
    Pronunciation
    
    </a>
    <a href='https://dictionary.cambridge.org/'
    class="px-2 py-4 inline-flex">

    <Book class="size-8 mr-2" />
        
        
        Dictionary</a>
    <a href='https://www.thesaurus.com/'
    class="px-2 py-4 inline-flex">

    <BookCopy class="size-8 mr-2" />
    Thesaurus
    
    
    </a>
</div>

<style>
    a:hover {
        @apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
    }
</style>