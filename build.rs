fn main() {
    let src = [
        "p4/overwatch.p4",
        "p4/parse.p4",
        "p4/core.p4",
        "p4/watchnpu.p4",
        "p4/headers.p4",
    ];
    for x in src {
        println!("cargo:rerun-if-changed={x}");
    }
}
