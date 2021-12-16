<script>
    import {onMount} from 'svelte';
    import init, { add } from 'algos-ds-wasm';

    let canvas;

    // WASM Magic
const maths = async () => {
  await init();

  console.log('Addition in rust:', add(1, 2));
};

    onMount(() => {
        maths();

        const ctx = canvas.getContext('2d');
        ctx.beginPath();
        ctx.arc(145, 75, 40, 0, 2 * Math.PI);
        ctx.stroke();

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
        <button class="genNewArr bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded">New Array</button>
        <button class="mergeSort bg-transparent hover:bg-blue-700 text-blue-500 font-semibold hover:text-white py-1.5 px-4 my-1 border border-blue-500 hover:border-transparent rounded">Merge Sort</button>
        <label for="speed" class="text-blue-500 font-bold">Set Animation Speed:</label>
        <input type="range" name="speed" id="speed" min="0" max="10" value="0" class="py-2 ">
    </div>
    <canvas style="border: 1px solid black;" bind:this={canvas}>
    </canvas>

</div>