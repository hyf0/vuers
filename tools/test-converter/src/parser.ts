import { parse, Lang, type SgNode } from '@ast-grep/napi';
import * as fs from 'fs';
import type { TestCase, Assertion, ParseResult } from './types.js';

/**
 * Parse a Vue test file and extract test cases
 */
export function parseTestFile(filePath: string): ParseResult {
  const source = fs.readFileSync(filePath, 'utf-8');
  const root = parse(Lang.TypeScript, source).root();

  const testCases: TestCase[] = [];
  const errors: string[] = [];

  // Find all describe blocks to build the hierarchy
  const describeBlocks = findDescribeBlocks(root, source);

  // Find all test/it blocks
  const testBlocks = findTestBlocks(root, source);

  for (const testBlock of testBlocks) {
    try {
      const testCase = extractTestCase(testBlock, describeBlocks, source);
      if (testCase) {
        testCases.push(testCase);
      }
    } catch (e) {
      errors.push(`Error parsing test at line ${testBlock.lineNumber}: ${e}`);
    }
  }

  return { filePath, testCases, errors };
}

interface DescribeBlock {
  name: string;
  startOffset: number;
  endOffset: number;
  lineNumber: number;
}

interface TestBlock {
  node: SgNode;
  name: string;
  lineNumber: number;
  bodyNode: SgNode | null;
}

/**
 * Find all describe() blocks and their ranges
 */
function findDescribeBlocks(root: SgNode, source: string): DescribeBlock[] {
  const blocks: DescribeBlock[] = [];

  // Pattern to match describe('name', () => { ... })
  const describeNodes = root.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'identifier',
        regex: '^describe$',
      },
    },
  });

  for (const node of describeNodes) {
    const args = node.child(1); // arguments node
    if (!args) continue;

    const children = args.children();
    // First argument should be the string name
    const nameNode = children.find(c =>
      c.kind() === 'string' || c.kind() === 'template_string'
    );

    if (nameNode) {
      const name = extractStringValue(nameNode);
      const range = node.range();
      blocks.push({
        name,
        startOffset: range.start.index,
        endOffset: range.end.index,
        lineNumber: range.start.line + 1,
      });
    }
  }

  return blocks;
}

/**
 * Find all test() and it() blocks
 */
function findTestBlocks(root: SgNode, source: string): TestBlock[] {
  const blocks: TestBlock[] = [];

  // Pattern to match test('name', ...) or it('name', ...)
  const testNodes = root.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'identifier',
        regex: '^(test|it)$',
      },
    },
  });

  for (const node of testNodes) {
    const args = node.child(1); // arguments node
    if (!args) continue;

    const children = args.children();
    // First argument should be the string name
    const nameNode = children.find(c =>
      c.kind() === 'string' || c.kind() === 'template_string'
    );

    // Second argument should be the callback function
    const callbackNode = children.find(c =>
      c.kind() === 'arrow_function' || c.kind() === 'function_expression'
    );

    if (nameNode) {
      const name = extractStringValue(nameNode);
      const range = node.range();
      blocks.push({
        node,
        name,
        lineNumber: range.start.line + 1,
        bodyNode: callbackNode ?? null,
      });
    }
  }

  return blocks;
}

/**
 * Extract a TestCase from a test block
 */
function extractTestCase(
  testBlock: TestBlock,
  describeBlocks: DescribeBlock[],
  source: string
): TestCase | null {
  const { node, name, lineNumber, bodyNode } = testBlock;

  // Build describe path by finding which describe blocks contain this test
  const testOffset = node.range().start.index;
  const describePath = describeBlocks
    .filter(d => d.startOffset < testOffset && d.endOffset > testOffset)
    .sort((a, b) => a.startOffset - b.startOffset)
    .map(d => d.name);

  // Extract SFC source from compile() or parse() calls
  const sfcSource = bodyNode ? extractSfcSource(bodyNode, source) : null;

  // Extract compile options
  const compileOptions = bodyNode ? extractCompileOptions(bodyNode) : {};

  // Extract assertions
  const assertions = bodyNode ? extractAssertions(bodyNode, source) : [];

  // Check if test expects an error
  // Patterns that indicate error-expecting:
  // 1. Body checks for errors (errors.length, errors[0], toThrow)
  // 2. Test name has "should throw", "should error" (but NOT "should not throw/error")
  // 3. Test name has "should only allow" (testing that violations produce errors)
  const bodyExpectsError = bodyNode ? checkExpectsError(bodyNode) : false;
  const nameExpectsError = (/should (throw|error|warn|fail)/i.test(name) && !/should not/i.test(name)) ||
    /should only allow/i.test(name);
  const expectsError = bodyExpectsError || nameExpectsError;

  // Get raw body for reference
  const rawBody = bodyNode?.text();

  return {
    name,
    describePath,
    lineNumber,
    sfcSource,
    compileOptions,
    assertions,
    expectsError,
    rawBody,
  };
}

/**
 * Extract the SFC source from compile() or parse() calls
 */
function extractSfcSource(bodyNode: SgNode, source: string): string | null {
  // Look for compile(`...`) or parse(`...`) calls
  const callNodes = bodyNode.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'identifier',
        regex: '^(compile|parse|compileSFCScript)$',
      },
    },
  });

  for (const callNode of callNodes) {
    const args = callNode.child(1);
    if (!args) continue;

    const children = args.children();
    // First argument should be the template string
    const templateNode = children.find(c =>
      c.kind() === 'template_string' || c.kind() === 'string'
    );

    if (templateNode) {
      return extractStringValue(templateNode);
    }
  }

  return null;
}

/**
 * Extract compile options from the second argument of compile() calls
 */
function extractCompileOptions(bodyNode: SgNode): Record<string, unknown> {
  const callNodes = bodyNode.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'identifier',
        regex: '^(compile|parse|compileSFCScript)$',
      },
    },
  });

  for (const callNode of callNodes) {
    const args = callNode.child(1);
    if (!args) continue;

    const children = args.children();
    // Second argument should be the options object
    const optionsNode = children.find(c => c.kind() === 'object');

    if (optionsNode) {
      try {
        // Try to parse the options object
        return parseObjectLiteral(optionsNode);
      } catch {
        // If parsing fails, return empty object
        return {};
      }
    }
  }

  return {};
}

/**
 * Extract assertions from expect() calls
 */
function extractAssertions(bodyNode: SgNode, source: string): Assertion[] {
  const assertions: Assertion[] = [];

  // Find expect().toMatch(), expect().toStrictEqual(), etc.
  const expectCalls = bodyNode.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'member_expression',
        has: {
          kind: 'call_expression',
          has: {
            kind: 'identifier',
            regex: '^expect$',
          },
        },
      },
    },
  });

  for (const expectCall of expectCalls) {
    const assertion = parseExpectAssertion(expectCall, source);
    if (assertion) {
      assertions.push(assertion);
    }
  }

  // Find assertCode() calls
  const assertCodeCalls = bodyNode.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'identifier',
        regex: '^assertCode$',
      },
    },
  });

  for (const assertCodeCall of assertCodeCalls) {
    const args = assertCodeCall.child(1);
    if (!args) continue;

    const children = args.children();
    const targetNode = children.find(c => c.kind() === 'identifier');

    if (targetNode) {
      assertions.push({
        type: 'snapshot',
        target: targetNode.text(),
        raw: assertCodeCall.text(),
      });
    }
  }

  return assertions;
}

/**
 * Parse an expect() assertion call
 */
function parseExpectAssertion(node: SgNode, source: string): Assertion | null {
  const memberExpr = node.child(0);
  if (!memberExpr || memberExpr.kind() !== 'member_expression') return null;

  // Get the method name (toMatch, toStrictEqual, etc.)
  const methodNode = memberExpr.child(2);
  if (!methodNode) return null;
  const method = methodNode.text();

  // Get the expect() call to find the target
  const expectCall = memberExpr.child(0);
  if (!expectCall) return null;

  const expectArgs = expectCall.child(1);
  if (!expectArgs) return null;

  // Find the target - can be identifier, member_expression, or subscript_expression
  const targetNode = expectArgs.children().find(c =>
    c.kind() === 'identifier' ||
    c.kind() === 'member_expression' ||
    c.kind() === 'subscript_expression'
  );
  const target = targetNode?.text() ?? 'unknown';

  // Skip assertions with unknown targets
  if (target === 'unknown') return null;

  // Get the assertion arguments
  const assertArgs = node.child(1);
  const expectedNode = assertArgs?.children().find(c =>
    c.kind() === 'string' || c.kind() === 'template_string' || c.kind() === 'object'
  );
  const expected = expectedNode?.text();

  // Map Jest/Vitest matchers to our assertion types
  let type: Assertion['type'];
  switch (method) {
    case 'toMatch':
    case 'toContain':
      type = 'contains';
      break;
    case 'toMatchSnapshot':
    case 'toMatchInlineSnapshot':
      type = 'snapshot';
      break;
    case 'toStrictEqual':
    case 'toEqual':
    case 'toBe':
      type = 'equals';
      break;
    case 'toThrow':
    case 'toThrowError':
      type = 'throws';
      break;
    case 'toBeTruthy':
      type = 'truthy';
      break;
    case 'toBeFalsy':
    case 'toBeUndefined':
    case 'toBeNull':
      type = 'falsy';
      break;
    default:
      // Skip unsupported matchers
      return null;
  }

  return {
    type,
    target,
    expected: expected ? extractStringValue({ text: () => expected, kind: () => expectedNode?.kind() ?? '' } as SgNode) : undefined,
    raw: node.text(),
  };
}

/**
 * Check if the test expects an error
 * Detects:
 * 1. expect(...).toThrow()
 * 2. expect(errors.length).toBe(N) where N > 0
 * 3. expect(errors[0]).toMatch(...) or .toContain(...)
 */
function checkExpectsError(bodyNode: SgNode): boolean {
  // Check for toThrow
  const throwCalls = bodyNode.findAll({
    rule: {
      kind: 'call_expression',
      has: {
        kind: 'member_expression',
        has: {
          kind: 'property_identifier',
          regex: '^(toThrow|toThrowError)$',
        },
      },
    },
  });

  if (throwCalls.length > 0) return true;

  // Check for errors.length assertions
  const bodyText = bodyNode.text();
  if (bodyText.includes('errors.length') && bodyText.includes('.toBe(') && !bodyText.includes('.toBe(0)')) {
    return true;
  }

  // Check for errors[0] or errors[1] assertions (accessing specific errors)
  if (/errors\[\d+\]/.test(bodyText) && (bodyText.includes('.toMatch') || bodyText.includes('.toContain'))) {
    return true;
  }

  return false;
}

/**
 * Check if the test is in a describe block related to warnings/errors
 */
function checkDescribePathIndicatesError(describePath: string[]): boolean {
  return describePath.some(path =>
    /warning|error/i.test(path)
  );
}

/**
 * Extract string value from a string or template_string node
 */
function extractStringValue(node: SgNode): string {
  const text = node.text();
  const kind = node.kind();

  if (kind === 'template_string') {
    // Remove backticks
    let content = text.slice(1, -1);
    // Check if the template string has interpolations
    if (content.includes('${')) {
      // Mark as having interpolations for filtering later
      // We'll skip tests with complex interpolations
      return `[INTERPOLATED]${content}`;
    }
    return content;
  } else if (kind === 'string') {
    // Remove quotes
    return text.slice(1, -1);
  }

  return text;
}

/**
 * Parse an object literal node into a JS object
 */
function parseObjectLiteral(node: SgNode): Record<string, unknown> {
  const result: Record<string, unknown> = {};

  const properties = node.findAll({
    rule: {
      kind: 'property',
    },
  });

  // Simplified: just capture key-value pairs as strings for now
  for (const prop of properties) {
    const children = prop.children();
    const keyNode = children.find(c =>
      c.kind() === 'property_identifier' || c.kind() === 'string'
    );
    const valueNode = children.find(c =>
      c.kind() !== 'property_identifier' && c.kind() !== 'string' && c.kind() !== ':'
    );

    if (keyNode && valueNode) {
      const key = extractStringValue(keyNode);
      result[key] = valueNode.text();
    }
  }

  return result;
}
