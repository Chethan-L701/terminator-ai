use cc;

fn main() {
    cc::Build::new().file("src/run.c").compile("run");
}
