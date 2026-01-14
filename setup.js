#!/usr/bin/env node
import { $, cd } from 'zx';

console.log('\n=== Initializing submodules ===');
await $`git submodule update --init`;

cd('hermes');

console.log('\n=== Configuring hermes build ===');
await $`cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release`;

console.log('\n=== Building hermes (this may take a while) ===');
await $`cmake --build ./build`;

console.log('\n=== Setup complete ===');
