use wasm_bindgen::prelude::*;

use md_porter_core::{CoreError, convert};

/// Converts a CoreError into a JsValue error carrying a stable `code` field.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Converts Markdown string to safe HTML with the supported GFM extensions.
///
/// Returns the HTML string on success, or a JsValue error with `code` and `message` fields.
#[wasm_bindgen(js_name = wasmMdToHtml)]
pub fn wasm_md_to_html(markdown: &str) -> Result<String, JsValue> {
    convert::md_to_html(markdown).map_err(core_err)
}

/// Strips all Markdown formatting and returns plain text.
#[wasm_bindgen(js_name = wasmMdToPlainText)]
pub fn wasm_md_to_plain_text(markdown: &str) -> Result<String, JsValue> {
    convert::md_to_plain_text(markdown).map_err(core_err)
}

/// Extracts YAML frontmatter from a Markdown string.
///
/// Returns a JS object with key-value pairs on success,
/// or `null` if no frontmatter is present.
#[wasm_bindgen(js_name = wasmExtractFrontmatter)]
pub fn wasm_extract_frontmatter(markdown: &str) -> Result<JsValue, JsValue> {
    let result = convert::extract_frontmatter(markdown).map_err(core_err)?;
    match result {
        Some(map) => {
            let obj = js_sys::Object::new();
            for (key, value) in map {
                js_sys::Reflect::set(&obj, &key.into(), &value.into()).ok();
            }
            Ok(obj.into())
        }
        None => Ok(JsValue::NULL),
    }
}

/// Converts Markdown into a self-contained HTML document.
///
/// Uses the default GitHub-like CSS if `css` is empty.
#[wasm_bindgen(js_name = wasmWrapMarkdownDocument)]
pub fn wasm_wrap_markdown_document(
    markdown: &str,
    title: &str,
    css: &str,
) -> Result<String, JsValue> {
    convert::wrap_markdown_document(markdown, title, css).map_err(core_err)
}

/// Returns the default embedded CSS used for exported HTML documents.
#[wasm_bindgen(js_name = wasmGetDefaultCss)]
pub fn wasm_get_default_css() -> String {
    convert::DEFAULT_CSS.to_string()
}

/// Initialization hook called by JS on module load.
/// Sets up console error panic hook for better debugging.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
