import { compileTemplate } from '@vue/compiler-sfc';

// Single template compile
globalThis.compile = function(template) {
  try {
    const result = compileTemplate({
      source: template,
      filename: 'template.vue',
      id: 'template'
    });
    return result.code;
  } catch (e) {
    return 'ERROR: ' + e.message;
  }
};

// Batch compile - takes JSON array, returns JSON array
globalThis.compileBatch = function(templatesJson) {
  const templates = JSON.parse(templatesJson);
  const results = templates.map(t => {
    try {
      return compileTemplate({
        source: t,
        filename: 'template.vue',
        id: 'template'
      }).code;
    } catch (e) {
      return 'ERROR: ' + e.message;
    }
  });
  return JSON.stringify(results);
};
