#!/usr/bin/env node

import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';
import { parseTestFile } from './parser.js';
import { generateRustTests, generateModRs } from './generator.js';
import type { GeneratedTest } from './types.js';

// Configuration
const VUE_TESTS_DIR = '../../../vue/packages/compiler-sfc/__tests__';
const VUE_SUBMODULE_DIR = '../../../vue';
const OUTPUT_DIR = '../../../src/tests/generated';

/**
 * Get the current commit hash of the Vue submodule
 */
function getVueCommitHash(vueDir: string): string {
  try {
    const hash = execSync(`git -C "${vueDir}" rev-parse HEAD`, { encoding: 'utf-8' }).trim();
    return hash;
  } catch (error) {
    console.warn('Warning: Could not get Vue commit hash, using "main"');
    return 'main';
  }
}

// Test files to process (start with simpler ones)
const TEST_FILES = [
  'parse.spec.ts',
  // 'compileScript.spec.ts',
  // 'compileStyle.spec.ts',
  // 'compileTemplate.spec.ts',
];

async function main() {
  const args = process.argv.slice(2);

  // Resolve paths relative to this script
  const scriptDir = path.dirname(new URL(import.meta.url).pathname);
  const vueTestsDir = path.resolve(scriptDir, VUE_TESTS_DIR);
  const vueSubmoduleDir = path.resolve(scriptDir, VUE_SUBMODULE_DIR);
  const outputDir = path.resolve(scriptDir, OUTPUT_DIR);

  // Get Vue commit hash for stable URLs
  const vueCommitHash = getVueCommitHash(vueSubmoduleDir);
  const sourceBaseUrl = `https://github.com/vuejs/core/blob/${vueCommitHash}/packages/compiler-sfc/__tests__`;

  console.log('Test Converter - Vue tests to Rust');
  console.log('===================================');
  console.log(`Vue tests directory: ${vueTestsDir}`);
  console.log(`Vue commit: ${vueCommitHash.substring(0, 7)}`);
  console.log(`Output directory: ${outputDir}`);
  console.log('');

  // Ensure output directory exists
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
    console.log(`Created output directory: ${outputDir}`);
  }

  // Determine which files to process
  let filesToProcess = TEST_FILES;
  if (args.length > 0) {
    filesToProcess = args.map(f => f.endsWith('.spec.ts') ? f : `${f}.spec.ts`);
  }

  const generatedModules: GeneratedTest[] = [];
  let totalTests = 0;
  let totalErrors = 0;

  for (const fileName of filesToProcess) {
    const filePath = path.join(vueTestsDir, fileName);

    if (!fs.existsSync(filePath)) {
      console.error(`File not found: ${filePath}`);
      continue;
    }

    console.log(`Processing: ${fileName}`);

    // Parse the test file
    const parseResult = parseTestFile(filePath);

    if (parseResult.errors.length > 0) {
      console.warn(`  Warnings:`);
      for (const error of parseResult.errors) {
        console.warn(`    - ${error}`);
      }
      totalErrors += parseResult.errors.length;
    }

    console.log(`  Found ${parseResult.testCases.length} test cases`);

    // Show some stats about what was found
    const withSfc = parseResult.testCases.filter(t => t.sfcSource !== null);
    const withAssertions = parseResult.testCases.filter(t => t.assertions.length > 0);
    console.log(`  - With SFC source: ${withSfc.length}`);
    console.log(`  - With assertions: ${withAssertions.length}`);

    // Generate Rust tests
    const generated = generateRustTests(parseResult.testCases, fileName, { sourceBaseUrl });

    if (generated.testCount > 0) {
      generatedModules.push(generated);
      totalTests += generated.testCount;

      // Write the generated file
      const outputPath = path.join(outputDir, `${generated.moduleName}.rs`);
      fs.writeFileSync(outputPath, generated.code);
      console.log(`  Generated ${generated.testCount} tests -> ${generated.moduleName}.rs`);
    } else {
      console.log(`  No tests generated (no SFC sources found)`);
    }

    console.log('');
  }

  // Generate mod.rs
  if (generatedModules.length > 0) {
    const modRs = generateModRs(generatedModules.map(m => m.moduleName));
    const modRsPath = path.join(outputDir, 'mod.rs');
    fs.writeFileSync(modRsPath, modRs);
    console.log(`Generated mod.rs with ${generatedModules.length} modules`);
  }

  // Summary
  console.log('');
  console.log('Summary');
  console.log('-------');
  console.log(`Files processed: ${filesToProcess.length}`);
  console.log(`Total tests generated: ${totalTests}`);
  console.log(`Modules created: ${generatedModules.length}`);
  if (totalErrors > 0) {
    console.log(`Warnings: ${totalErrors}`);
  }
}

// Run the main function
main().catch(err => {
  console.error('Error:', err);
  process.exit(1);
});
