/**
 * Represents an assertion in a test case
 */
export interface Assertion {
  /** Type of assertion */
  type: 'snapshot' | 'contains' | 'not_contains' | 'equals' | 'throws' | 'truthy' | 'falsy';
  /** Target of the assertion (e.g., 'content', 'bindings', 'errors') */
  target: string;
  /** Expected value for the assertion */
  expected?: string;
  /** Raw assertion code for reference */
  raw?: string;
}

/**
 * Represents a single test case extracted from a Vue test file
 */
export interface TestCase {
  /** Test name from it() or test() */
  name: string;
  /** Path of nested describe() blocks */
  describePath: string[];
  /** Line number in the source file */
  lineNumber: number;
  /** SFC source code if found (from compile() or parse() calls) */
  sfcSource: string | null;
  /** Options passed to compile/parse functions */
  compileOptions: Record<string, unknown>;
  /** List of assertions in the test */
  assertions: Assertion[];
  /** Whether the test expects an error/throw */
  expectsError: boolean;
  /** Raw test body for reference */
  rawBody?: string;
}

/**
 * Result of parsing a test file
 */
export interface ParseResult {
  /** Source file path */
  filePath: string;
  /** Extracted test cases */
  testCases: TestCase[];
  /** Any parsing errors encountered */
  errors: string[];
}

/**
 * Options for the generator
 */
export interface GeneratorOptions {
  /** Base URL for source links */
  sourceBaseUrl: string;
  /** Module name for the generated test file */
  moduleName: string;
}

/**
 * Generated Rust test output
 */
export interface GeneratedTest {
  /** Module name (e.g., 'parse', 'compile_script') */
  moduleName: string;
  /** Generated Rust code */
  code: string;
  /** Number of tests generated */
  testCount: number;
}
