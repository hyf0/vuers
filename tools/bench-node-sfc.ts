/**
 * Benchmark: Full SFC compilation using Node.js + @vue/compiler-sfc
 *
 * Run: npx tsx tools/bench-node-sfc.ts
 */

import { parse, compileScript, compileTemplate, compileStyle } from '@vue/compiler-sfc';
import { readFileSync } from 'fs';

const WARMUP = 10;
const ITERATIONS = 100;

const source = readFileSync('examples/fixtures/App.vue', 'utf-8');
const filename = 'App.vue';
const scopeId = 'benchmark123';

function compileSfc(): { js: string; css: string } {
  // 1. Parse
  const { descriptor } = parse(source, { filename });

  // 2. Compile script
  const scriptResult = compileScript(descriptor, {
    id: scopeId,
    isProd: false,
  });

  // 3. Compile template
  const templateResult = compileTemplate({
    source: descriptor.template?.content || '',
    filename,
    id: scopeId,
    scoped: descriptor.styles.some(s => s.scoped),
    compilerOptions: {
      bindingMetadata: scriptResult.bindings,
    },
  });

  // 4. Compile styles
  const cssResults = descriptor.styles.map(style =>
    compileStyle({
      source: style.content,
      filename,
      id: scopeId,
      scoped: style.scoped,
    })
  );

  return {
    js: `${scriptResult.content}\n${templateResult.code}`,
    css: cssResults.map(r => r.code).join('\n'),
  };
}

console.log('=== Node.js SFC Compilation Benchmark ===\n');
console.log(`Warmup: ${WARMUP} iterations`);
console.log(`Benchmark: ${ITERATIONS} iterations\n`);

// Warmup
for (let i = 0; i < WARMUP; i++) {
  compileSfc();
}

// Benchmark
const start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  compileSfc();
}
const duration = performance.now() - start;

const perOp = duration / ITERATIONS;
const throughput = (ITERATIONS / duration) * 1000;

console.log(`Total: ${duration.toFixed(2)}ms`);
console.log(`Per operation: ${perOp.toFixed(3)}ms`);
console.log(`Throughput: ${Math.round(throughput)} ops/sec`);
