#[hoare::hoare(precond="true")]
fn trivial_1() {}
#[test]
fn test_trivial_1() { trivial_1(); }

#[hoare::hoare(postcond="true")]
fn trivial_2() {}
#[test]
fn test_trivial_2() { trivial_2(); }

#[hoare::hoare(precond="true", postcond="true", invariant="true")]
fn trivial_4() {}
#[test]
fn test_trivial_4() { trivial_4(); }

#[hoare::hoare(precond="false")]
fn fail_trivial_1() {}
#[test]
#[should_panic]
fn test_fail_trivial_1() {
    fail_trivial_1();
}

#[hoare::hoare(postcond="result")]
fn trt() -> bool { true }
#[test]
fn test_result_trivial() { trt(); }

#[hoare::hoare(postcond="result")]
fn trtf() -> bool { false }
#[test]
#[should_panic]
fn test_result_trivial_fail() { trtf(); }

#[hoare::hoare(postcond="result > 0")]
fn tr1(x: i32) -> i32 { x }
#[test]
fn test_result_1() { tr1(5); }

#[test]
#[should_panic]
fn test_result_1_fail() { tr1(-5); }

#[hoare::hoare(postcond="result == 'a'")]
fn tr2(x: char) -> char { x }
#[test]
fn test_result_2() { tr2('a'); }
#[test]
#[should_panic]
fn test_result_2_fail() { tr2('b'); }

#[hoare::hoare(postcond="result > 5")]
fn tr3(path: bool) -> i32 {
    if path {
        return 42;
    }
    10
}
#[test]
fn test_result3() {
    tr3(true);
    tr3(false);
}

struct Bar {
    f1: i32,
    f2: i32,
}
#[hoare::hoare(invariant="x.f1 < x.f2")]
fn baz(x: &mut Bar) {
    x.f1 += 10;
    x.f2 += 10;
}
#[test]
fn test_bar_struct_field_access() {
    let mut bar = Bar { f1: 0, f2: 10 };
    baz(&mut bar);
    assert_eq!(10, bar.f1);
    assert_eq!(20, bar.f2);
}
