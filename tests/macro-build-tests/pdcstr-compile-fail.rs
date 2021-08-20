fn main() {
    // with internal nul 
    netcorehost::pdcstr!("\0");
    netcorehost::pdcstr!("somerandomteststring\0");
    netcorehost::pdcstr!("somerandomteststring\0somerandomteststring");
    netcorehost::pdcstr!("somerandomteststring\0somerandomteststring");
}
