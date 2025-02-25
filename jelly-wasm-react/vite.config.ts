import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';

const resolvePath = (str: string) => path.resolve(__dirname, str);

export default defineConfig({
    define: {
        global: 'window',
    },
    css: {
        preprocessorOptions: {
            scss: {
                api: 'modern-compiler',
            },
        },
    },
    plugins: [react(), dts({ tsconfigPath: resolvePath('./tsconfig.build.json') })],
    build: {
        outDir: 'lib',
        lib: {
            entry: resolvePath('packages/index.ts'),
            name: 'index',
            fileName: (format) => `index.${format}.js`,
        },
        rollupOptions: {
            external: ['react', 'react-dom'],
            output: {
                globals: {
                    react: 'react',
                    'react-dom': 'react-dom',
                },
            },
        },
    },
});
