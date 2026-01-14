// Measure cold start - compile once and exit
import { compileTemplate } from '@vue/compiler-sfc';

const template = `<div class="container"><span>{{ msg }}</span><button @click="handleClick">Click me</button></div>`;

const start = performance.now();
const result = compileTemplate({ source: template, filename: 'test.vue', id: 'test' });
const duration = performance.now() - start;

console.log(`First compile: ${duration.toFixed(2)}ms`);
