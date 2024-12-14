<script lang="ts">

export let input = 'Regular **bold** and *italic* text with inline math: $E = mc^2$';
export let textArea:HTMLTextAreaElement;


function insertAtCursor(text: string) {
    const start = textArea.selectionStart;
    const end = textArea.selectionEnd;
    const beforeText = input.substring(0, start);
    const afterText = input.substring(end);
    const selectedText = input.substring(start, end);

    input = beforeText + text + afterText;
    
    // Set cursor position after inserted text
    setTimeout(() => {
        textArea.focus();
        textArea.setSelectionRange(start + text.length, start + text.length);
    }, 0);
}

const mathSymbols = [
    { label: 'α', tex: '\\alpha' },
    { label: 'β', tex: '\\beta' },
    { label: 'γ', tex: '\\gamma' },
    { label: '∑', tex: '\\sum' },
    { label: '∫', tex: '\\int' },
    { label: '√', tex: '\\sqrt{}' },
    { label: '±', tex: '\\pm' },
    { label: '∞', tex: '\\infty' },
    { label: '→', tex: '\\to' },
    { label: '≤', tex: '\\leq' },
    { label: '≥', tex: '\\geq' },
    { label: '≠', tex: '\\neq' },
];

const markdownShortcuts = [
    { label: 'Bold', insert: '**text**' },
    { label: 'Italic', insert: '*text*' },
    { label: 'H1', insert: '# ' },
    { label: 'H2', insert: '## ' },
    { label: 'List', insert: '- ' },
    { label: 'Chem', insert: '$\\ce{}$' },
];

</script>
<div class="toolbar">
    <div class="toolbar-section">
        {#each markdownShortcuts as shortcut}
            <button class="p-1 hover:text-orangedeuterium-700" on:click|preventDefault={() => insertAtCursor(shortcut.insert)}>
                {shortcut.label}
            </button>
        {/each}
    </div>
    <div class="toolbar-section">
        {#each mathSymbols as symbol}
            <button class="p-1 hover:text-orangedeuterium-700" on:click|preventDefault={() => insertAtCursor('$' + symbol.tex + '$')}
                    title={symbol.tex}>
                {symbol.label}
            </button>
        {/each}
    </div>
</div>
