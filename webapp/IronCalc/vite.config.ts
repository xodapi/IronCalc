import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import svgr from 'vite-plugin-svgr';
import { resolve } from 'node:path';
import { copyFileSync, mkdirSync, existsSync } from 'node:fs';

// Plugin to copy WASM files to dist
function copyWasmPlugin() {
  return {
    name: 'copy-wasm',
    writeBundle() {
      const wasmSrc = resolve(__dirname, '../../bindings/wasm/pkg');
      const distDir = resolve(__dirname, 'dist');

      try {
        // Copy WASM files
        if (existsSync(`${wasmSrc}/wasm_bg.wasm`)) {
          copyFileSync(`${wasmSrc}/wasm_bg.wasm`, `${distDir}/wasm_bg.wasm`);
          console.log('✓ Copied wasm_bg.wasm');
        }
        if (existsSync(`${wasmSrc}/wasm.js`)) {
          copyFileSync(`${wasmSrc}/wasm.js`, `${distDir}/wasm.js`);
          console.log('✓ Copied wasm.js');
        }
      } catch (e) {
        console.warn('Warning: Could not copy WASM files:', e.message);
      }
    }
  };
}

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  // For Tauri build, we build as app with HTML entry
  const isTauriBuild = process.env.TAURI_ENV_DEBUG !== undefined ||
    process.env.npm_lifecycle_event === 'tauri:build';

  return {
    build: isTauriBuild ? {
      // Build as application for Tauri
      outDir: 'dist',
      emptyDir: true,
      rollupOptions: {
        input: resolve(__dirname, 'index.html'),
      },
    } : {
      // Build as library for npm
      lib: {
        entry: resolve(__dirname, 'src/index.ts'),
        name: 'IronCalc',
        fileName: 'ironcalc',
      },
      rollupOptions: {
        external: ['react', 'react-dom', '@ironcalc/wasm'],
        output: {
          globals: {
            react: 'React',
            'react-dom': 'ReactDOM',
            '@ironcalc/wasm': 'IronCalc',
          },
        },
      },
    },
    plugins: [react(), svgr(), copyWasmPlugin()],
    server: {
      fs: {
        allow: ['..', '../../bindings/wasm/pkg'],
      },
    },
  };
});
