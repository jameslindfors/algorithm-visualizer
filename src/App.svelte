<script>
    import {onMount} from 'svelte';
    import init, { bubble_sort, canvas_image} from 'algos-ds-wasm';

    let canvas;
    let arr = [];
    let sorted;

    // WASM Magic
const bubbleSort = async () => {
    await init();
    sorted = bubble_sort(arr);
};

const draw = async () => {
    await init();
    canvas_image();
};

const gen_rand_arr = () => {
    for (let i = 0; i < 20; i++) {
        arr[i] = Math.floor(Math.random() * 100);
    }
    sorted = undefined;
}

    onMount(() => {
        gen_rand_arr();
        draw();
    });
</script>
<svelte:head>
    <title>Home</title>
</svelte:head>

<div class="app">
    <header>
        <h2 class="font-bold text-3xl mx-4 py-4">Sorting Algorithms</h2>   
    </header>
    <div class="controls container mx-4 px-1 flex flex-col w-44">
        <button class="genNewArr bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded" on:click={() => gen_rand_arr()}>New Array</button>
        <button class="mergeSort bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded" on:click={() => bubbleSort()}>Bubble Sort</button>
        <label for="speed" class="text-blue-500 font-bold">Set Animation Speed:</label>
        <input type="range" name="speed" id="speed" min="0" max="10" value="0" class="py-2 ">
    </div>
    <canvas style="border: 1px solid black;" id="canvas" class="mx-4 py-4" bind:this={canvas}>

    </canvas>

    <div class="container mx-4">
        <h3 class="text-2xl mx-2 py-4 text-fuchsia-400 font-semibold underline">Starting Array: </h3>
        {arr}
        {#if sorted}
        <h3 class="text-2xl mx-2 py-4 text-purple-400 font-semibold underline">Sorted: </h3>
        {sorted}
        {/if}
    </div>

</div>