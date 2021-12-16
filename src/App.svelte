<script>
    import init, { add } from 'algos-ds-wasm';

const maths = async () => {
  await init();

  console.log('Addition in rust:', add(1, 2));
};

maths();
</script>
<svelte:head>
    <title>Home</title>
</svelte:head>

<div class="app">
    <header>
        <h1 class="text-blue-600 text-5xl">
            Snowpack Svelte
        </h1>
    </header>

</div>