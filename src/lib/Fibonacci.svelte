<script>
    import { invoke } from '@tauri-apps/api/tauri';

    /** @type {number} */
    let nth;

    /** @type {HTMLInputElement} */
    let inputField;
    
    /** @type {string} */
    let result = '';

    async function fib() {
        result = await invoke('fib', { nth });
    }

    /** @param {KeyboardEvent} event */
    async function onKeydown(event) {
        if (event.key !== 'Enter') return;
        await fib();
        inputField.value = '';
    }

    async function clearInput() {
        inputField.value = '';
        result = '';
    }
</script>

<div class="container">
    <h1>Fibonacci Calculator</h1>
    
    <div class="row">
        <input 
            type="number"
            min="0"
            bind:this={inputField} 
            id="fib-input"
            placeholder="Enter a number..."
            bind:value="{nth}"
            on:keydown={onKeydown} />
        <button on:click="{fib}">Calculate</button>
        <button id="clear-fields" on:click="{clearInput}">Clear</button>
    </div>
    
    <div class="row" id="result">
        <p>{result}</p>
    </div>
</div>