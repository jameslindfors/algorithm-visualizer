<script>
    import {onMount} from 'svelte';
    import init, {start} from 'algos-ds-wasm';

    let arr = undefined;
    let sorted = undefined;

    const wasm_module = init();

    onMount (() => {
        wasm_module.then(async () => {
            start();
        });
    });

</script>
<svelte:head>
    <title>Sorting</title>
</svelte:head>

<div class="app  h-full w-full">
    <header>
        <h2 class="font-bold text-3xl mx-4 py-4">Sorting Algorithms</h2>   
    </header>
    <!-- <div class="controls container mx-4 px-1 flex flex-col w-44">
        <button class="genNewArr bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded" on:click={() => gen_rand_arr()}>New Array</button>
        <button class="mergeSort bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded" on:click={() => bubbleSort()}>Bubble Sort</button>
        <label for="speed" class="text-blue-500 font-bold">Set Animation Speed:</label>
        <input type="range" name="speed" id="speed" min="0" max="10" value="0" class="py-2 ">
    </div> -->
    <canvas style="border: 0.5px solid white;" id="canvas" class="mx-4">
    </canvas>

    <div class="container mx-4">
        <h3 class="text-2xl mx-2 py-1 font-semibold underline">Starting Array: </h3>
        <h4 class="mx-2"> {arr}</h4>
        {#if sorted}
        <h3 class="text-2xl mx-2 py-1 font-semibold underline">Sorted Array: </h3>
        <h4 class="mx-2">{sorted}</h4>
        {/if}
    </div>
    
</div>