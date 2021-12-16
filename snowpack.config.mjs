/** @type {import("snowpack").SnowpackUserConfig } */
export default {
  mount: {
    /* ... */
    // directory name :'build directory'
    public: "/",
    src: "/dist",
  },
  plugins: [
    /* ... */
    "@snowpack/plugin-svelte",
    "@snowpack/plugin-postcss",

    [
      "snowpack-plugin-wasm-pack",
      {
        projectPath: "./algos-ds-wasm",
      },
    ],
  ],
  routes: [
    /* Enable an SPA Fallback in development: */
    // {"match": "routes", "src": ".*", "dest": "/index.html"},
  ],
  optimize: {
    /* Example: Bundle your final build: */
    // "bundle": true,
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
    tailwindConfig: "/tailwind.config.js",
  },
  buildOptions: {
    /* ... */
  },
};
