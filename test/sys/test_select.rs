use nix::sys::select::{FdSet, FD_SETSIZE, select};
use nix::sys::time::{TimeVal, TimeValLike};
use nix::unistd::{write, pipe};

#[test]
fn test_fdset() {
    let mut fd_set = FdSet::new();

    for i in 0..FD_SETSIZE {
        assert!(!fd_set.contains(i));
    }

    fd_set.insert(7);

    assert!(fd_set.contains(7));

    fd_set.remove(7);

    for i in 0..FD_SETSIZE {
        assert!(!fd_set.contains(i));
    }

    fd_set.insert(1);
    fd_set.insert(FD_SETSIZE / 2);
    fd_set.insert(FD_SETSIZE - 1);

    fd_set.clear();

    for i in 0..FD_SETSIZE {
        assert!(!fd_set.contains(i));
    }
}

// powerpc-unknown-linux-gnu currently fails on the first `assert_eq` because
// `select()` returns a 0 instead of a 1. Since this test has only been run on
// qemu, it's unclear if this is a OS or qemu bug. Just disable it on that arch
// for now.
// FIXME: Fix tests for powerpc
// FIXME: Add a link to an upstream qemu bug if there is one
#[test]
#[cfg_attr(target_arch = "powerpc", ignore)]
fn test_select() {
    let (r1, w1) = pipe().unwrap();
    write(w1, b"hi!").unwrap();
    let (r2, _w2) = pipe().unwrap();

    let mut fd_set = FdSet::new();
    fd_set.insert(r1);
    fd_set.insert(r2);

    let mut timeout = TimeVal::seconds(10);
    assert_eq!(1, select(r2 + 1,
                         Some(&mut fd_set),
                         None,
                         None,
                         Some(&mut timeout)).unwrap());
    assert!(fd_set.contains(r1));
    assert!(!fd_set.contains(r2));
}
