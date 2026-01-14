#include <stdlib.h>
#include <string.h>
#include <memory>
#include <hermes/VM/static_h.h>
#include <hermes/hermes.h>
#include <jsi/jsi.h>

// Declaration for the `vue_compiler` unit created by Static Hermes.
extern "C" SHUnit sh_export_vue_compiler;

// Cached runtime and functions
static SHRuntime *s_shRuntime = nullptr;
static facebook::hermes::HermesRuntime *s_hermes = nullptr;
// Use raw pointers and intentionally leak to avoid destruction order issues
static facebook::jsi::Function *s_compileFn = nullptr;
static facebook::jsi::Function *s_compileBatchFn = nullptr;

static void init_runtime() {
  if (s_shRuntime == nullptr) {
    s_shRuntime = _sh_init(0, nullptr);
    s_hermes = _sh_get_hermes_runtime(s_shRuntime);
    if (!_sh_initialize_units(s_shRuntime, 1, &sh_export_vue_compiler)) {
      abort();
    }
    auto global = s_hermes->global();
    s_compileFn = new facebook::jsi::Function(
      global.getPropertyAsFunction(*s_hermes, "compile"));
    s_compileBatchFn = new facebook::jsi::Function(
      global.getPropertyAsFunction(*s_hermes, "compileBatch"));
  }
}

extern "C" char* vue_compile_template(const char *template_str) {
  init_runtime();
  auto& rt = *s_hermes;
  auto jsResult = s_compileFn->call(rt, facebook::jsi::String::createFromUtf8(rt, template_str));
  auto resultStr = jsResult.getString(rt).utf8(rt);
  char* result = (char*)malloc(resultStr.size() + 1);
  memcpy(result, resultStr.data(), resultStr.size() + 1);
  return result;
}

extern "C" char* vue_compile_batch(const char *templates_json) {
  init_runtime();
  auto& rt = *s_hermes;
  auto jsResult = s_compileBatchFn->call(rt, facebook::jsi::String::createFromUtf8(rt, templates_json));
  auto resultStr = jsResult.getString(rt).utf8(rt);
  char* result = (char*)malloc(resultStr.size() + 1);
  memcpy(result, resultStr.data(), resultStr.size() + 1);
  return result;
}

extern "C" void vue_free_string(char *ptr) {
  free(ptr);
}
