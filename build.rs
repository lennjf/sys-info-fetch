fn main() {
    // actions
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "sys-info-fetch.gresource",
    );
    
}
