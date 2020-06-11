use deno_core::plugin_api::*;

// All ops are registered here
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("connect", op_connect);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
