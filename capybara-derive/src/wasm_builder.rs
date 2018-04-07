use super::BindingBuilder;

pub struct WasmBuilder;

impl WasmBuilder {
    fn actual_impl(&self, input: String) -> String {
        quote!(use capybara::reexport::*;).into_string() + "#[wasm_bindgen]" + &input
    }
}

impl BindingBuilder for WasmBuilder {
    fn class(&self, _attr: String, input: String) -> String {
        self.actual_impl(input)
    }

    fn methods(&self, _attr: String, input: String) -> String {
        self.actual_impl(input)
    }

    fn foreign_mod(&self, _: String, input: String) -> String {
        self.actual_impl(input)
    }
}
