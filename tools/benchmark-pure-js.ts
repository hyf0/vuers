// Measure pure JS compile time inside Hermes (no FFI)
import { compileTemplate } from "@vue/compiler-sfc";

const template = `<div class="container"><span>{{ msg }}</span><button @click="handleClick">Click me</button></div>`;
const iterations = 1000;

// Warmup
for (let i = 0; i < 10; i++) {
  compileTemplate({ source: template, filename: "test.vue", id: "test" });
}

// Benchmark
const start = performance.now();
for (let i = 0; i < iterations; i++) {
  const r = compileTemplate({ source: template, filename: "test.vue", id: "test" });
  r.code;
}
const duration = performance.now() - start;

console.log("Pure JS (Node/V8):");
console.log(`  Compiles/sec: ${Math.round(iterations / (duration / 1000))}`);
console.log(`  Avg: ${(duration / iterations).toFixed(3)}ms`);
