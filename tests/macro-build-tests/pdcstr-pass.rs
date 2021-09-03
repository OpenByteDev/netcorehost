fn main() {
    let _ = netcorehost::pdcstr!("");
    let _ = netcorehost::pdcstr!("test");
    let _ = netcorehost::pdcstr!("test with spaces");
    let _ = netcorehost::pdcstr!("0");
    let _ = netcorehost::pdcstr!("\\0");
    let _ = netcorehost::pdcstr!("κόσμε");
}
