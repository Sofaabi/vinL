use vinl::os::process;

//This test should ideally spawn a child process with a known name and pid, then find it. Currently this at least works fine on systemd running systems such as Arch.
#[test]
fn can_find_systemd(){
    let x = process::get_pid("systemd");
    let x = x.unwrap();
    let x = &x[0];
    assert!(x.contains("1"));
}