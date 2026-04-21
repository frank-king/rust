use std::marker::ValueSized; //~ ERROR use of unstable library feature `value_sized`

fn main() {
    let _: &dyn ValueSized = &(); //~ ERROR use of unstable library feature `value_sized`
}
