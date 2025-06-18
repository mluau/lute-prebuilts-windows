pub fn integrate() {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR")).unwrap();
    lute_src_rs_common::prebuilts::build_prebuilt_default(lute_src_rs_common::LConfig::default());
}