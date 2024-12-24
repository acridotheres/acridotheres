use std::{collections::HashMap, path::PathBuf};

fn main() {
    let aliases = HashMap::from(
        [("images", "assets/images"), ("fonts", "assets/fonts")]
            .map(|(k, v)| (k.to_string(), PathBuf::from(v))),
    );

    let cfg = slint_build::CompilerConfiguration::new().with_library_paths(aliases);

    slint_build::compile_with_config("ui/app-window.slint", cfg).expect("Slint build failed");
}
