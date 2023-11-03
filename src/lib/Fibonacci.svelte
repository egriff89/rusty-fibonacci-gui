<script>
    import { invoke } from '@tauri-apps/api/tauri';

    /** @type {number} */
    let nth;

    /** @type {HTMLInputElement} */
    let inputField;
    
    /** @type {string} */
    let result;

    async function fib() {
        result = await invoke('fib', { nth });
    }

    /** @param {KeyboardEvent} event */
    async function onKeydown(event) {
        if (event.key !== 'Enter') return;
        await fib();
        inputField.value = '';
    }
</script>

<div class="container">
    <h1>Fibonacci Calculator</h1>
    
    <div class="row">
        <input 
            type="number"
            bind:this={inputField} 
            id="fib-input"
            placeholder="Enter a number..."
            bind:value="{nth}"
            on:keydown={onKeydown} />
        <button on:click="{fib}">Calculate</button>
    </div>
    
    {#if result}
        <p>{result}</p>
    {:else}
        <p></p>
    {/if}
</div>