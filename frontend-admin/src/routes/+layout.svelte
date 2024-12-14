<script>
    import "../app.css";
    import { page } from "$app/stores";



</script>

<div class="flex flex-col text-tealdeuterium-950 bg-floral-50 w-screen h-screen">
    <slot />
</div>
<h1 class="text-[6rem] fixed bottom-0 font-oswald text-tealdeuterium-700 px-8">
    editor
</h1>

<style lang="postcss">
    

    :global(.preview h1) {
        font-size: 1.8em;
        font-weight: 800;
        margin: 0.8em 0;
    }

    :global(.preview h2) {
        font-size: 1.4em;
        margin: 0.7em 0;
    }

    :global(.preview p) {
        margin: 1em 0;
    }

    :global(.preview ul) {
        list-style-type: disc;
    }

    :global(.preview ol) {
        list-style-type: decimal;
    }

    :global(.preview a) {
        color: #CC6D0E;
        text-decoration: underline;
    }

    :global(.inline-math) {
        display: inline-flex;
        align-items: center;
        vertical-align: middle;
        padding: 0 0.1em;
    }

    :global(.inline-math .katex) {
        font-size: 1.1em;
    }

    :global(.katex-display) {
        margin: 1em 0;
        overflow-x: auto;
        overflow-y: hidden;
    }

    :global(.preview ul, .preview ol) {
        margin: 1em 0;
        padding-left: 2em;
    }
  </style>