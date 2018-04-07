use super::BindingBuilder;

/// This is stub target that does not emitt any bindings
pub struct StubBuilder;

impl BindingBuilder for StubBuilder {
    fn class(&self, _: String, input: String) -> String {
        input
    }

    fn methods(&self, _: String, input: String) -> String {
        input
    }

    fn foreign_mod(&self, _: String, input: String) -> String {
        input
    }
}
