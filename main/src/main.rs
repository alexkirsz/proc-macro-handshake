use registry::{list, register};

register!(::plugin);

fn main() {
    let plugins = list!();

    for (name, init_data, call_data) in plugins {
        println!(
            "crate `{name}` registered with init data `{init_data}` and call data `{call_data}`"
        );
    }
}
