#![feature(rustc_private)]
#[macro_use]
mod common;
use common::*;

const COMMON: &str = code_str! {
    use vstd::*;
    fn require_sync<T: Sync>(t: T) { }
    fn require_send<T: Send>(t: T) { }

    ::builtin_macros::verus!{
        struct Pred<A, B> { a: A, b: B }
        impl<A, B> vstd::invariant::InvariantPredicate<A, B> for Pred<A, B> {
            open spec fn inv(k: A, v: B) -> bool { true }
        }
    }
};

#[macro_export]
macro_rules! check_send_sync {
    ($name:ident, $tparams:expr, $t:expr) => {
        test_verify_one_file! {
            #[test] $name COMMON.to_string() + &"
                fn test1$tparams(t: $t) {
                    require_sync(t);
                }
                fn test2$tparams(t: $t) {
                    require_send(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Ok(())
        }
    };
}

#[macro_export]
macro_rules! check_send {
    ($name:ident, $name2:ident, $tparams:expr, $t:expr) => {
        test_verify_one_file! {
            #[test] $name COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_send(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Ok(())
        }
        test_verify_one_file! {
            #[test] $name2 COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_sync(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Err(e) => assert_rust_error_msg_all(e, "between threads safely")
        }
    };
}

#[macro_export]
macro_rules! check_sync {
    ($name:ident, $name2:ident, $tparams:expr, $t:expr) => {
        test_verify_one_file! {
            #[test] $name COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_sync(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Ok(())
        }
        test_verify_one_file! {
            #[test] $name2 COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_send(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Err(e) => assert_rust_error_msg_all(e, "between threads safely")
        }
    };
}

#[macro_export]
macro_rules! check_none {
    ($name:ident, $name2:ident, $tparams:expr, $t:expr) => {
        test_verify_one_file! {
            #[test] $name COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_send(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Err(e) => assert_rust_error_msg_all(e, "between threads safely")
        }
        test_verify_one_file! {
            #[test] $name2 COMMON.to_string() + &"
                fn test2$tparams(t: $t) {
                    require_sync(t);
                }
                ".replace("$tparams", $tparams)
                .replace("$t", $t)
            => Err(e) => assert_rust_error_msg_all(e, "between threads safely")
        }
    };
}

// raw ptrs

check_send_sync!(raw_ptr_points_to_send_sync, "<T: Send + Sync>", "vstd::raw_ptr::PointsTo<T>");
check_send!(
    raw_ptr_points_to_send,
    raw_ptr_points_to_send2,
    "<T: Send>",
    "vstd::raw_ptr::PointsTo<T>"
);
check_sync!(
    raw_ptr_points_to_sync,
    raw_ptr_points_to_sync2,
    "<T: Sync>",
    "vstd::raw_ptr::PointsTo<T>"
);
check_none!(raw_ptr_points_none, raw_ptr_points_none2, "<T>", "vstd::raw_ptr::PointsTo<T>");

// PPtr

check_send_sync!(ptr_points_to_send_sync, "<T: Send + Sync>", "vstd::ptr::PointsTo<T>");
check_send!(ptr_points_to_send, ptr_points_to_send2, "<T: Send>", "vstd::ptr::PointsTo<T>");
check_sync!(ptr_points_to_sync, ptr_points_to_sync2, "<T: Sync>", "vstd::ptr::PointsTo<T>");
check_none!(ptr_points_none, ptr_points_none2, "<T>", "vstd::ptr::PointsTo<T>");

check_send_sync!(points_to_raw_send_sync, "", "vstd::ptr::PointsToRaw");

// cells

check_send_sync!(cell_points_to_send_sync, "<T: Send + Sync>", "vstd::cell::PointsTo<T>");
check_send!(cell_points_to_send, cell_points_to_send2, "<T: Send>", "vstd::cell::PointsTo<T>");
check_sync!(cell_points_to_sync, cell_points_to_sync2, "<T: Sync>", "vstd::cell::PointsTo<T>");
check_none!(cell_points_none, cell_points_none2, "<T>", "vstd::cell::PointsTo<T>");

check_send_sync!(pcell, "<T: Send + Sync>", "vstd::cell::PCell<T>");

// LocalInvariant

check_send!(
    local_send_sync,
    local_send_sync2,
    "<T: Send + Sync>",
    "vstd::invariant::LocalInvariant<(), T, Pred<(), T>>"
);
check_send!(
    local_send,
    local_send2,
    "<T: Send>",
    "vstd::invariant::LocalInvariant<(), T, Pred<(), T>>"
);
check_none!(
    local_sync,
    local_sync2,
    "<T: Sync>",
    "vstd::invariant::LocalInvariant<(), T, Pred<(), T>>"
);
check_none!(local_none, local_none2, "<T>", "vstd::invariant::LocalInvariant<(), T, Pred<(), T>>");

// AtomicInvariant

check_send_sync!(
    atomic_send_sync,
    "<T: Send + Sync>",
    "vstd::invariant::AtomicInvariant<(), T, Pred<(), T>>"
);
check_send_sync!(atomic_send, "<T: Send>", "vstd::invariant::AtomicInvariant<(), T, Pred<(), T>>");
check_none!(
    atomic_sync,
    atomic_sync2,
    "<T: Sync>",
    "vstd::invariant::AtomicInvariant<(), T, Pred<(), T>>"
);
check_none!(
    atomic_none,
    atomic_none2,
    "<T>",
    "vstd::invariant::AtomicInvariant<(), T, Pred<(), T>>"
);
