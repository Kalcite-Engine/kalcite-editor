use std::{env, fs, path::PathBuf};

fn main() {
    let source = PathBuf::from("src/editor_core.klc");
    println!("cargo:rerun-if-changed={}", source.display());
    let text = fs::read_to_string(&source).expect("read editor KLC core");
    let syntax = kalcite_syntax::parse(&text).expect("parse editor KLC core");
    let hir = kalcite_hir::lower(&syntax).expect("lower editor KLC core");
    let mir = kalcite_mir::lower(&hir);
    let emitted = kalcite_backend_rust::emit_library(&mir, "").expect("emit editor KLC core");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("editor_core.rs"),
        emitted,
    )
    .expect("write generated editor KLC core");
}
