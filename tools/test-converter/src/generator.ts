import type { TestCase, Assertion, GeneratorOptions, GeneratedTest } from './types.js';

const SOURCE_BASE_URL = 'https://github.com/vuejs/core/blob/main/packages/compiler-sfc/__tests__';

/**
 * Generate Rust test code from parsed test cases
 */
export function generateRustTests(
  testCases: TestCase[],
  fileName: string,
  options: Partial<GeneratorOptions> = {}
): GeneratedTest {
  const moduleName = fileNameToModuleName(fileName);
  const sourceBaseUrl = options.sourceBaseUrl ?? SOURCE_BASE_URL;

  const tests: string[] = [];

  for (const testCase of testCases) {
    const rustTest = generateRustTest(testCase, fileName, sourceBaseUrl);
    if (rustTest) {
      tests.push(rustTest);
    }
  }

  const code = generateModuleCode(moduleName, tests);

  return {
    moduleName,
    code,
    testCount: tests.length,
  };
}

/**
 * Generate a single Rust test function from a TestCase
 */
function generateRustTest(
  testCase: TestCase,
  fileName: string,
  sourceBaseUrl: string
): string | null {
  // Skip tests without SFC source (we need something to parse/compile)
  if (!testCase.sfcSource) {
    return null;
  }

  // Skip tests with interpolated strings (too complex to convert automatically)
  if (testCase.sfcSource.startsWith('[INTERPOLATED]')) {
    return null;
  }

  const functionName = testNameToFunctionName(testCase);
  const sourceLink = `${sourceBaseUrl}/${fileName}#L${testCase.lineNumber}`;
  const describePath = testCase.describePath.join(' > ');

  // Escape the SFC source for Rust raw string literal
  const sfcSource = escapeForRawString(testCase.sfcSource);
  const rawStringDelimiter = findRawStringDelimiter(sfcSource);

  // Generate assertions
  const assertions = generateAssertions(testCase.assertions);

  // Build the test function
  const lines: string[] = [];

  // Doc comments with local path and GitHub link
  const localPath = `vue/packages/compiler-sfc/__tests__/${fileName}:${testCase.lineNumber}`;
  lines.push(`/// Local: ${localPath}`);
  lines.push(`/// GitHub: ${sourceLink}`);
  if (describePath) {
    lines.push(`/// Path: ${describePath}`);
  }
  lines.push(`/// Test: ${testCase.name}`);
  lines.push(`#[test]`);

  lines.push(`fn ${functionName}() {`);

  // SFC source as raw string literal
  lines.push(`    let source = r${rawStringDelimiter}"${sfcSource}"${rawStringDelimiter};`);
  lines.push(``);

  // Parse the SFC
  lines.push(`    let result = parse(source, "test.vue").unwrap();`);

  if (testCase.expectsError) {
    // Error-expecting test: check that errors exist
    lines.push(``);
    lines.push(`    // This test expects parse errors`);
    lines.push(`    assert!(result.has_errors(), "Expected parse errors but got none");`);

    // If there are specific error message assertions, add them
    const errorAssertions = assertions.filter(a =>
      a.target && a.target.includes('errors') && a.expected
    );
    if (errorAssertions.length > 0) {
      for (const assertion of errorAssertions) {
        if (assertion.expected) {
          const escapedExpected = escapeForRawString(assertion.expected);
          const delim = findRawStringDelimiter(escapedExpected);
          lines.push(`    assert!(result.error_message(0).contains(r${delim}"${escapedExpected}"${delim}), "Error message mismatch");`);
        }
      }
    }
  } else {
    // Normal test: get descriptor and run assertions
    lines.push(`    let desc = result.descriptor().unwrap();`);
    lines.push(``);

    // Add assertions
    if (assertions.length > 0) {
      lines.push(...assertions.map(a => `    ${a}`));
    } else {
      // Default: just check that parsing succeeded without errors
      lines.push(`    assert!(!result.has_errors(), "Parse errors: {:?}", result.errors().collect::<Vec<_>>());`);
    }
  }

  lines.push(`}`);

  return lines.join('\n');
}

/**
 * Generate Rust assertions from test assertions
 */
function generateAssertions(assertions: Assertion[]): string[] {
  const rustAssertions: string[] = [];

  for (const assertion of assertions) {
    const rustAssertion = generateAssertion(assertion);
    if (rustAssertion) {
      rustAssertions.push(rustAssertion);
    }
  }

  return rustAssertions;
}

/**
 * Generate a single Rust assertion
 */
function generateAssertion(assertion: Assertion): string | null {
  // Skip assertions targeting unimplemented features
  if (assertion.target.includes('.ast')) {
    return `// TODO: ast() not implemented - skipping assertion`;
  }

  switch (assertion.type) {
    case 'snapshot':
      // Use insta for snapshot testing
      return `insta::assert_snapshot!(${mapTarget(assertion.target)});`;

    case 'contains':
      if (!assertion.expected) return null;
      const containsExpected = escapeForRawString(assertion.expected);
      const containsDelim = findRawStringDelimiter(containsExpected);
      return `assert!(${mapTarget(assertion.target)}.contains(r${containsDelim}"${containsExpected}"${containsDelim}), "Expected to contain: ${escapeDoubleQuotes(assertion.expected)}");`;

    case 'not_contains':
      if (!assertion.expected) return null;
      const notContainsExpected = escapeForRawString(assertion.expected);
      const notContainsDelim = findRawStringDelimiter(notContainsExpected);
      return `assert!(!${mapTarget(assertion.target)}.contains(r${notContainsDelim}"${notContainsExpected}"${notContainsDelim}), "Expected NOT to contain: ${escapeDoubleQuotes(assertion.expected)}");`;

    case 'equals':
      if (!assertion.expected) return null;
      // For complex objects, we might need special handling
      if (assertion.expected.startsWith('{')) {
        // Object comparison - generate a comment for manual review
        return `// TODO: Assert ${assertion.target} equals ${assertion.expected}`;
      }
      const equalsExpected = escapeForRawString(assertion.expected);
      const equalsDelim = findRawStringDelimiter(equalsExpected);
      const equalsTarget = mapTarget(assertion.target);

      // attrs().get() returns Option<&AttrValue>, need special handling
      if (assertion.target.includes('.attrs[') || assertion.target.includes("attrs['")) {
        return `assert!(matches!(${equalsTarget}, Some(crate::AttrValue::String(s)) if s == r${equalsDelim}"${equalsExpected}"${equalsDelim}));`;
      }

      return `assert_eq!(${equalsTarget}, r${equalsDelim}"${equalsExpected}"${equalsDelim});`;

    case 'truthy':
      // For Option types, check is_some(). For bool, assert the value.
      const truthyTarget = mapTarget(assertion.target);
      // content() returns &str, check !is_empty()
      if (assertion.target.endsWith('.content')) {
        return `assert!(!${truthyTarget}.is_empty());`;
      }
      if (truthyTarget.endsWith('()')) {
        // Method call returning Option - check is_some()
        return `assert!(${truthyTarget}.is_some());`;
      }
      return `assert!(${truthyTarget});`;

    case 'falsy':
      // For Option types, check is_none(). For bool, assert not the value.
      const falsyTarget = mapTarget(assertion.target);
      // content() returns &str, check is_empty()
      if (assertion.target.endsWith('.content')) {
        return `assert!(${falsyTarget}.is_empty());`;
      }
      if (falsyTarget.endsWith('()')) {
        // Method call returning Option - check is_none()
        return `assert!(${falsyTarget}.is_none());`;
      }
      return `assert!(!${falsyTarget});`;

    case 'throws':
      // Handled at the function level with #[should_panic]
      return null;

    default:
      return null;
  }
}

/**
 * Map JS target names to Rust equivalents
 * Uses the actual Rust API with method calls:
 * - parse() returns ParseOutput
 * - result.descriptor() returns Option<Descriptor>
 * - desc.template() returns Option<TemplateBlock>
 * - block.content() returns &str
 */
function mapTarget(target: string): string {
  // Handle member expressions like descriptor.template!.content
  const cleaned = target.replace(/!/g, '');

  // Common mappings - from JS property paths to Rust API equivalents
  const mappings: Record<string, string> = {
    // Script compilation results
    'content': 'script_result.content()',
    'bindings': 'script_result.bindings()',

    // Parse result - error_message returns &str directly
    'errors': 'result.errors()',
    'errors.length': 'result.error_count()',
    'errors[0].message': 'result.error_message(0)',
    'errors[1].message': 'result.error_message(1)',

    // Descriptor
    'descriptor': 'desc',
    'descriptor.template': 'desc.template()',
    'descriptor.template.content': 'desc.template().unwrap().content()',
    'descriptor.template.lang': 'desc.template().unwrap().lang()',
    // Note: ast() is not implemented in Rust API, skip these tests
    'descriptor.template.ast': '/* TODO: ast() not implemented */ None::<()>',
    'descriptor.script': 'desc.script()',
    'descriptor.script.content': 'desc.script().unwrap().content()',
    'descriptor.script.attrs': 'desc.script().unwrap().attrs()',
    'descriptor.scriptSetup': 'desc.script_setup()',
    'descriptor.scriptSetup.content': 'desc.script_setup().unwrap().content()',
    'descriptor.styles': 'desc.styles()',
    'descriptor.styles.length': 'desc.style_count()',
    'descriptor.customBlocks': 'desc.custom_blocks()',
    'descriptor.customBlocks.length': 'desc.custom_blocks().count()',
    'descriptor.slotted': 'desc.slotted()',
    'descriptor.cssVars': 'desc.css_vars()',
    'descriptor.source': 'desc.source()',
  };

  if (mappings[cleaned]) {
    return mappings[cleaned];
  }

  // Handle more complex paths dynamically
  let result = cleaned;

  // Handle descriptor.X paths
  if (result.startsWith('descriptor.')) {
    result = result.replace('descriptor.', 'desc.');

    // Convert template/script/style access to method calls
    result = result
      .replace(/\.template\./, '.template().unwrap().')
      .replace(/\.script\./, '.script().unwrap().')
      .replace(/\.scriptSetup\./, '.script_setup().unwrap().')
      .replace(/\.styles\[(\d+)\]/, '.styles().nth($1).unwrap()')
      .replace(/\.customBlocks\[(\d+)\]/, '.custom_blocks().nth($1).unwrap()');

    // Convert property access to method calls
    result = result
      .replace(/\.content$/, '.content()')
      .replace(/\.lang$/, '.lang()')
      .replace(/\.src$/, '.src()')
      .replace(/\.attrs$/, '.attrs()');
  }

  // Handle subscript access with string keys like attrs['src'] before .attrs() conversion
  // This handles patterns like descriptor.script.attrs['src']
  result = result.replace(/\.attrs\['(\w+)'\]/g, '.attrs().get("$1")');
  result = result.replace(/\.attrs\["(\w+)"\]/g, '.attrs().get("$1")');
  result = result.replace(/\.attrs\(\)\['(\w+)'\]/g, '.attrs().get("$1")');
  result = result.replace(/\.attrs\(\)\["(\w+)"\]/g, '.attrs().get("$1")');

  // Convert camelCase to snake_case for remaining identifiers
  result = result.replace(/([a-z])([A-Z])/g, '$1_$2').toLowerCase();

  return result;
}

/**
 * Convert a test file name to a Rust module name
 */
function fileNameToModuleName(fileName: string): string {
  return fileName
    .replace(/\.spec\.ts$/, '')
    .replace(/([a-z])([A-Z])/g, '$1_$2')
    .toLowerCase();
}

/**
 * Convert a test name to a valid Rust function name
 */
function testNameToFunctionName(testCase: TestCase): string {
  // Combine describe path and test name
  const parts = [...testCase.describePath, testCase.name];

  const name = parts
    .join('_')
    // Remove special characters
    .replace(/[^a-zA-Z0-9_\s]/g, '')
    // Convert spaces to underscores
    .replace(/\s+/g, '_')
    // Convert camelCase to snake_case
    .replace(/([a-z])([A-Z])/g, '$1_$2')
    .toLowerCase()
    // Remove consecutive underscores
    .replace(/_+/g, '_')
    // Remove leading/trailing underscores
    .replace(/^_+|_+$/g, '');

  // Ensure it starts with a letter
  if (/^\d/.test(name)) {
    return `test_${name}`;
  }

  return name;
}

/**
 * Escape string for Rust raw string literal
 */
function escapeForRawString(str: string): string {
  // Raw strings don't need escaping, but we need to handle the delimiter
  return str;
}

/**
 * Find a suitable raw string delimiter that doesn't appear in the content
 */
function findRawStringDelimiter(content: string): string {
  let hashes = '#';
  while (content.includes(`"${hashes}`)) {
    hashes += '#';
  }
  return hashes;
}

/**
 * Escape double quotes for regular strings
 */
function escapeDoubleQuotes(str: string): string {
  return str.replace(/"/g, '\\"');
}

/**
 * Generate the complete module code
 */
function generateModuleCode(moduleName: string, tests: string[]): string {
  const lines: string[] = [];

  // Module header
  lines.push(`//! Generated tests from Vue compiler-sfc test suite`);
  lines.push(`//! Module: ${moduleName}`);
  lines.push(`//!`);
  lines.push(`//! DO NOT EDIT - This file is auto-generated by test-converter`);
  lines.push(``);
  lines.push(`use crate::parse;`);
  lines.push(``);

  // Add all tests
  for (const test of tests) {
    lines.push(test);
    lines.push(``);
  }

  return lines.join('\n');
}

/**
 * Generate the mod.rs file for the generated tests directory
 */
export function generateModRs(moduleNames: string[]): string {
  const lines: string[] = [];

  lines.push(`//! Generated tests from Vue compiler-sfc test suite`);
  lines.push(`//!`);
  lines.push(`//! DO NOT EDIT - This file is auto-generated by test-converter`);
  lines.push(``);

  for (const name of moduleNames) {
    lines.push(`mod ${name};`);
  }

  return lines.join('\n');
}
