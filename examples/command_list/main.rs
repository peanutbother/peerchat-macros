use peerchat_macros::command_list;

fn main() {
    for command in MyEnum::VARIANTS.iter() {
        let current = match command.to_string().as_str() {
            | "A" => MyEnum::A,
            | "B" => MyEnum::B,
            | "C" => MyEnum::C,
            | "D" => MyEnum::D,
            | "E" => MyEnum::E,
        };
        println!(
            "MyEnum::{} description: {:?} hidden: {}",
            current.to_string(),
            current.description(),
            current.is_hidden()
        );
    }
}

#[derive(Debug)]
#[command_list]
pub enum MyEnum {
    /// providing `description` and `hidden`
    /// explicitly set hidden (default false)
    #[hidden = true]
    #[description = "this is A"]
    A,

    /// providing `description` and `hidden` with
    /// explicitly set hidden (default false)
    #[hidden = false]
    #[description = "this is B with an even longer description"]
    B,

    /// just providing `hidden`
    #[hidden = false]
    /// omit description to fall back to `None`
    C,

    #[doc = r#"omitting both `description` and `hidden` and commenting with different doc style"#]
    D,

    /**
       yet another doc style to debug
       setting a value to `variant` (non-unit type enum)
    */
    E = 8,
}
