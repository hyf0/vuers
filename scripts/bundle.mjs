import { rolldown } from 'rolldown';

const bundle = await rolldown({
  input: 'src/vue-compiler.js',
  platform: 'browser',
  resolve: {
    conditionNames: ['module', 'import', 'browser', 'default']
  }
});

await bundle.write({
  dir: 'dist',
  format: 'iife',
  name: 'VueCompiler'
});

console.log('Bundle created: dist/vue-compiler.js');
