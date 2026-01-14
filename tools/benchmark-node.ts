import { compileTemplate } from "@vue/compiler-sfc";

const template = `<div class="container"><span>{{ msg }}</span><button @click="handleClick">Click me</button></div>`;
const iterations = 1000;

// Warmup
for (let i = 0; i < 10; i++) {
  const r = compileTemplate({ source: template, filename: "test.vue", id: "test" });
  r.code; // Access code to match native version
}

// Benchmark
const start = performance.now();
for (let i = 0; i < iterations; i++) {
  const r = compileTemplate({ source: template, filename: "test.vue", id: "test" });
  r.code; // Access code to match native version
}
const duration = performance.now() - start;

console.log("Node.js Benchmark:");
console.log(`  Iterations: ${iterations}`);
console.log(`  Total time: ${duration.toFixed(2)}ms`);
console.log(`  Avg per compile: ${(duration / iterations).toFixed(3)}ms`);
console.log(`  Compiles/sec: ${Math.round(iterations / (duration / 1000))}`);
