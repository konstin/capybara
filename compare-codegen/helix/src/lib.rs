#[macro_use]
extern crate helix;

ruby! {
    class ExportedClass {
        struct {
            number: i32
        }

        def initialize(helix, number: i32) {
            ExportedClass { helix, number }
        }

        def no_args() {}
        def one_arg(_a: i32) {}
        def no_args_returning() -> i32 {
            42
        }
        def one_arg_returning(_a: i32) -> i32 {
            42
        }

        def self_no_args_returning(&self) -> i32 {
            42
        }

        def self_mut_no_args_returning(&mut self) -> i32 {
            42
        }

        def get_number(&self) -> i32 {
            self.number
        }
    }
}
