fn main() {
    // with internal nul 
    let _ = netcorehost::pdcstr!("\0");
    let _ = netcorehost::pdcstr!("somerandomteststring\0");
    let _ = netcorehost::pdcstr!("somerandomteststring\0somerandomteststring");
    let _ = netcorehost::pdcstr!("somerandomteststring\0somerandomteststring");
}
