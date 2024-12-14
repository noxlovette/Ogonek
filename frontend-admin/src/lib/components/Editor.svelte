<script lang="ts">
    import katex from 'katex';
    import { marked } from 'marked';

    import 'katex/dist/katex.min.css';
    import 'katex/dist/contrib/mhchem';
    import { enhance } from '$app/forms';
    import markedKatex from "marked-katex-extension";
    import { getContext } from 'svelte';
    import Toolbar from './Toolbar.svelte';
    import Categories from './Categories.svelte';
    import Tagger from './Tagger.svelte';
    import { EyeOff, Eye } from 'lucide-svelte';
    import Difficulty from './Difficulty.svelte';

    let article:App.Article = getContext('article');
    let input = article?.content || '';
    let textArea:HTMLTextAreaElement;

    const options = {
        throwOnError: false
    };
    marked.use(markedKatex(options));

    // Sanitize and parse the content
    $: renderedContent = 
        marked.parse(
            input.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, "")
        );

    let hidden = false;
    let title = article?.title || '';
</script>


<form class="flex" method="post" action="?/update" use:enhance>

    <input type="hidden" name="id" value={article?.id} />
    
    <div class="grid grid-cols-2 gap-8">
        <div class="flex flex-col size-full">
        <input type="text" name="title" bind:value={title} placeholder="Title" class="p-2 border border-floral-200 rounded text-4xl font-bold font-space w-full focus:outline-none focus:ring-2 focus:ring-tealdeuterium-500" />
    <textarea 
    class="w-full h-full p-4 leading-6 border rounded font-space focus:outline-none focus:ring-2 focus:ring-tealdeuterium-500"
        bind:value={input}
        bind:this={textArea}
        placeholder="Type markdown and LaTeX here..."
    />
    </div>
    <input type="hidden" name="content" value={input} />
    <div>
        <h1 class="p-2 text-4xl font-bold font-oswald w-full">
            {title}
        </h1>
    <div class="preview px-8 py-2 leading-6 min-h-[300px] border border-floral-300 shadow-lg rounded">
        <input type="hidden" name="processed_html" value={renderedContent} />
        {@html renderedContent}
    </div>
</div>
</div>

<div id="controls" class="flex flex-col fixed right-10 bottom-10 max-w-[800px] bg-floral-100 shadow-lg rounded px-6 py-4 border-floral-200 border min-h-10"

>
    <button on:click|preventDefault={() => hidden = !hidden} class="absolute top-2 right-3">
        {#if !hidden}
        <EyeOff />
        {:else}
        <Eye />
        {/if}
    </button>

    <div class:hidden class="space-y-4">
    <button type="submit" class="font-bold text-2xl hover:text-orangedeuterium-700 border-b-2 w-full text-left border-floral-300">
        Save
    </button>
    <Toolbar textArea={textArea} bind:input />
    <Categories />
    <Difficulty />
    <Tagger />
</div>
</div>
</form>


