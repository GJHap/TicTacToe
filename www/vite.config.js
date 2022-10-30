import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { defineConfig } from 'vite'

export default defineConfig({
    esbuild: {
        jsxInject: `import React from 'react'`
    },
    root: 'src',
    build: {
        outDir: '../public',
    },
    server: {
        port: 8000,
        open: true
    },
    plugins: [
        wasm(),
        topLevelAwait()
    ]
});