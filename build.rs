use cc;

fn main() {
    cc::Build::new()
        .include("include")
        .file("src/tests.c")
        .warnings(true)
        .extra_warnings(true)
        .warnings_into_errors(true)
        .compile("ctests");
}
