//! Snapshot tests for Vue SFC parsing using insta.
//!
//! These tests verify that all SFC parsing fields are correctly populated
//! by comparing against expected snapshots.

// Allow dead_code for snapshot structs - fields are used by Debug derive for insta
#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::{Compiler, AttrValue, ImportBinding, Position, SourceLocation};

/// Serializable struct for Position (for deterministic output)
#[derive(Debug)]
struct PositionSnapshot {
    offset: usize,
    line: usize,
    column: usize,
}

impl From<Position> for PositionSnapshot {
    fn from(p: Position) -> Self {
        Self {
            offset: p.offset,
            line: p.line,
            column: p.column,
        }
    }
}

/// Serializable struct for SourceLocation
#[derive(Debug)]
struct SourceLocationSnapshot {
    start: PositionSnapshot,
    end: PositionSnapshot,
}

impl From<SourceLocation> for SourceLocationSnapshot {
    fn from(loc: SourceLocation) -> Self {
        Self {
            start: loc.start.into(),
            end: loc.end.into(),
        }
    }
}

/// Serializable struct for AttrValue
#[derive(Debug)]
enum AttrValueSnapshot {
    String(String),
    Bool(bool),
}

impl From<AttrValue> for AttrValueSnapshot {
    fn from(av: AttrValue) -> Self {
        match av {
            AttrValue::String(s) => AttrValueSnapshot::String(s),
            AttrValue::Bool(b) => AttrValueSnapshot::Bool(b),
        }
    }
}

/// Serializable struct for ImportBinding
#[derive(Debug)]
struct ImportBindingSnapshot {
    is_type: bool,
    imported: String,
    source: String,
    is_from_setup: bool,
}

impl From<ImportBinding> for ImportBindingSnapshot {
    fn from(ib: ImportBinding) -> Self {
        Self {
            is_type: ib.is_type,
            imported: ib.imported,
            source: ib.source,
            is_from_setup: ib.is_from_setup,
        }
    }
}

/// Serializable struct for template block
#[derive(Debug)]
struct TemplateBlockSnapshot {
    content: String,
    lang: String,
    src: Option<String>,
    loc: SourceLocationSnapshot,
    attrs: BTreeMap<String, AttrValueSnapshot>,
}

/// Serializable struct for script block
#[derive(Debug)]
struct ScriptBlockSnapshot {
    content: String,
    lang: String,
    src: Option<String>,
    loc: SourceLocationSnapshot,
    attrs: BTreeMap<String, AttrValueSnapshot>,
    is_setup: bool,
    setup_value: Option<String>,
    bindings: BTreeMap<String, String>,
    imports: BTreeMap<String, ImportBindingSnapshot>,
    warnings: Vec<String>,
    deps: Vec<String>,
}

/// Serializable struct for style block
#[derive(Debug)]
struct StyleBlockSnapshot {
    content: String,
    lang: String,
    src: Option<String>,
    loc: SourceLocationSnapshot,
    attrs: BTreeMap<String, AttrValueSnapshot>,
    is_scoped: bool,
    has_module: bool,
    module_name: Option<String>,
}

/// Serializable struct for custom block
#[derive(Debug)]
struct CustomBlockSnapshot {
    block_type: String,
    content: String,
    lang: String,
    src: Option<String>,
    loc: SourceLocationSnapshot,
    attrs: BTreeMap<String, AttrValueSnapshot>,
}

/// Serializable struct for descriptor
#[derive(Debug)]
struct DescriptorSnapshot {
    filename: String,
    source_length: usize,
    css_vars: Vec<String>,
    slotted: bool,
    custom_blocks_count: usize,
    template: Option<TemplateBlockSnapshot>,
    script: Option<ScriptBlockSnapshot>,
    script_setup: Option<ScriptBlockSnapshot>,
    styles: Vec<StyleBlockSnapshot>,
    custom_blocks: Vec<CustomBlockSnapshot>,
}

/// Serializable struct for parse output
#[derive(Debug)]
struct ParseOutputSnapshot {
    has_errors: bool,
    error_count: usize,
    errors: Vec<String>,
    descriptor: Option<DescriptorSnapshot>,
}

/// Convert HashMap to BTreeMap for deterministic ordering
fn to_btree_attrs(
    attrs: std::collections::HashMap<String, AttrValue>,
) -> BTreeMap<String, AttrValueSnapshot> {
    attrs
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect()
}

fn to_btree_bindings(
    bindings: std::collections::HashMap<String, String>,
) -> BTreeMap<String, String> {
    bindings.into_iter().collect()
}

fn to_btree_imports(
    imports: std::collections::HashMap<String, ImportBinding>,
) -> BTreeMap<String, ImportBindingSnapshot> {
    imports
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect()
}

/// Test parsing a complete SFC with all block types
#[test]
fn test_complete_sfc_parsing() {
    let source = r#"<template lang="pug">
div {{ count }}
</template>

<script lang="ts">
export default {
  name: 'TestComponent'
}
</script>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Ref } from 'vue'
const count = ref(0)
const double = computed(() => count.value * 2)
</script>

<style scoped>
div { color: v-bind(color); }
</style>

<style module="styles">
.container { display: flex; }
</style>

<i18n>
{ "en": { "hello": "Hello" } }
</i18n>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "TestComponent.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("complete_sfc", snapshot);
}

/// Test parsing a minimal SFC with only template
#[test]
fn test_minimal_template_only() {
    let source = r#"<template>
  <div>Hello World</div>
</template>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "Minimal.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("minimal_template_only", snapshot);
}

/// Test parsing SFC with script setup and ref/computed bindings
#[test]
fn test_script_setup_bindings() {
    let source = r#"<template>
  <div>{{ count }} - {{ doubled }} - {{ message }}</div>
</template>

<script setup>
import { ref, computed, reactive } from 'vue'

const count = ref(0)
const doubled = computed(() => count.value * 2)
const state = reactive({ message: 'Hello' })
const message = 'Static string'
let mutable = 1

defineProps(['title'])
defineEmits(['update'])
</script>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "ScriptSetup.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("script_setup_bindings", snapshot);
}

/// Test parsing SFC with external src attributes
#[test]
fn test_external_src_attributes() {
    let source = r#"<template src="./template.html"></template>
<script src="./script.ts" lang="ts"></script>
<style src="./style.css" scoped></style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "External.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("external_src_attributes", snapshot);
}

/// Test parsing SFC with multiple styles (scoped, module, plain)
#[test]
fn test_multiple_styles() {
    let source = r#"<template>
  <div class="container">Content</div>
</template>

<style>
/* Global styles */
body { margin: 0; }
</style>

<style scoped>
/* Scoped styles */
.container { padding: 10px; }
</style>

<style module>
/* CSS Modules with default $style */
.box { border: 1px solid; }
</style>

<style module="custom">
/* CSS Modules with custom name */
.item { display: block; }
</style>

<style lang="scss" scoped>
/* SCSS scoped */
$primary: #42b883;
.btn { color: $primary; }
</style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "MultiStyle.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("multiple_styles", snapshot);
}

/// Test parsing SFC with CSS v-bind() variables
#[test]
fn test_css_v_bind_variables() {
    let source = r#"<template>
  <div class="box">Styled Box</div>
</template>

<script setup>
const primaryColor = '#42b883'
const borderWidth = '2px'
const fontSize = '16px'
</script>

<style scoped>
.box {
  color: v-bind(primaryColor);
  border-width: v-bind(borderWidth);
  font-size: v-bind(fontSize);
  background: v-bind('primaryColor + "22"');
}
</style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "VBind.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("css_v_bind_variables", snapshot);
}

/// Test parsing SFC with :slotted() in scoped styles
#[test]
fn test_slotted_styles() {
    let source = r#"<template>
  <div class="wrapper">
    <slot></slot>
  </div>
</template>

<style scoped>
.wrapper {
  padding: 10px;
}

:slotted(.slot-content) {
  color: red;
}

:slotted(*) {
  margin: 0;
}
</style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "Slotted.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("slotted_styles", snapshot);
}

/// Test parsing SFC with multiple custom blocks
#[test]
fn test_custom_blocks() {
    let source = r#"<template>
  <div>{{ $t('greeting') }}</div>
</template>

<script setup>
const msg = 'Hello'
</script>

<i18n lang="json">
{
  "en": {
    "greeting": "Hello!"
  },
  "es": {
    "greeting": "Hola!"
  }
}
</i18n>

<docs>
# MyComponent

This is the documentation for MyComponent.

## Usage

```vue
<MyComponent />
```
</docs>

<story name="default">
Default story configuration
</story>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "CustomBlocks.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("custom_blocks", snapshot);
}

/// Test parsing SFC with type-only imports
#[test]
fn test_type_imports() {
    let source = r#"<template>
  <div>{{ value }}</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Ref, ComputedRef } from 'vue'
import { type PropType } from 'vue'

const value: Ref<number> = ref(42)
</script>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "TypeImports.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("type_imports", snapshot);
}

/// Test parsing SFC with both regular script and script setup
#[test]
fn test_dual_scripts() {
    let source = r#"<template>
  <div>{{ count }}</div>
</template>

<script lang="ts">
export default {
  name: 'DualScript',
  inheritAttrs: false
}
</script>

<script setup lang="ts">
import { ref } from 'vue'

const count = ref(0)
</script>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "DualScript.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("dual_scripts", snapshot);
}

/// Test parsing empty/whitespace content blocks
#[test]
fn test_empty_blocks() {
    let source = r#"<template></template>
<script></script>
<style></style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "Empty.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("empty_blocks", snapshot);
}

/// Test parsing SFC with various attribute formats
#[test]
fn test_attribute_formats() {
    let source = r#"<template functional>
  <div>Functional template</div>
</template>

<script lang="ts" generic="T extends string">
export default {}
</script>

<style scoped lang="scss" media="screen">
.test { color: red; }
</style>
"#;

    let compiler = Compiler::new().expect("Compiler should initialize");
    let result = compiler.parse(source, "Attributes.vue").expect("Parse should succeed");

    let snapshot = build_parse_output_snapshot(&result);
    insta::assert_debug_snapshot!("attribute_formats", snapshot);
}

/// Helper function to build a complete ParseOutputSnapshot
fn build_parse_output_snapshot(result: &crate::ParseOutput) -> ParseOutputSnapshot {
    ParseOutputSnapshot {
        has_errors: result.has_errors(),
        error_count: result.error_count(),
        errors: result.errors().map(|s| s.to_string()).collect(),
        descriptor: result.descriptor().map(|desc| DescriptorSnapshot {
            filename: desc.filename().to_string(),
            source_length: desc.source().len(),
            css_vars: desc.css_vars(),
            slotted: desc.slotted(),
            custom_blocks_count: desc.custom_blocks_count(),
            template: desc.template().map(|t| TemplateBlockSnapshot {
                content: t.content().to_string(),
                lang: t.lang().to_string(),
                src: t.src().map(|s| s.to_string()),
                loc: t.loc().into(),
                attrs: to_btree_attrs(t.attrs()),
            }),
            script: desc.script().map(|s| ScriptBlockSnapshot {
                content: s.content().to_string(),
                lang: s.lang().to_string(),
                src: s.src().map(|src| src.to_string()),
                loc: s.loc().into(),
                attrs: to_btree_attrs(s.attrs()),
                is_setup: s.is_setup(),
                setup_value: s.setup_value().map(|v| v.to_string()),
                bindings: to_btree_bindings(s.bindings()),
                imports: to_btree_imports(s.imports()),
                warnings: s.warnings(),
                deps: s.deps(),
            }),
            script_setup: desc.script_setup().map(|s| ScriptBlockSnapshot {
                content: s.content().to_string(),
                lang: s.lang().to_string(),
                src: s.src().map(|src| src.to_string()),
                loc: s.loc().into(),
                attrs: to_btree_attrs(s.attrs()),
                is_setup: s.is_setup(),
                setup_value: s.setup_value().map(|v| v.to_string()),
                bindings: to_btree_bindings(s.bindings()),
                imports: to_btree_imports(s.imports()),
                warnings: s.warnings(),
                deps: s.deps(),
            }),
            styles: desc
                .styles()
                .map(|s| StyleBlockSnapshot {
                    content: s.content().to_string(),
                    lang: s.lang().to_string(),
                    src: s.src().map(|src| src.to_string()),
                    loc: s.loc().into(),
                    attrs: to_btree_attrs(s.attrs()),
                    is_scoped: s.is_scoped(),
                    has_module: s.has_module(),
                    module_name: s.module_name().map(|n| n.to_string()),
                })
                .collect(),
            custom_blocks: desc
                .custom_blocks()
                .map(|cb| CustomBlockSnapshot {
                    block_type: cb.block_type().to_string(),
                    content: cb.content().to_string(),
                    lang: cb.lang().to_string(),
                    src: cb.src().map(|s| s.to_string()),
                    loc: cb.loc().into(),
                    attrs: to_btree_attrs(cb.attrs()),
                })
                .collect(),
        }),
    }
}
