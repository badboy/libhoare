struct Foo {
    x: i32,
}

impl Foo {
    #[hoare::hoare(precond="self.x < 10 && y < 10", postcond="self.x > 10", invariant="self.x < 15")]
    fn foo(&mut self, y: i32) {
        self.x += y;
    }

    #[hoare::hoare(postcond="result < 15")]
    fn foo_ret(&mut self, y: i32) -> i32 {
        y
    }
}

#[test]
fn test_impl_1(){
    let mut f = Foo { x: 5 };
    f.foo(6);
}

#[test]
#[should_panic]
fn test_impl_2(){
    let mut f = Foo { x: 10 };
    f.foo(4);
}

#[test]
#[should_panic]
fn test_impl_3(){
    let mut f = Foo { x: 9 };
    f.foo(9);
}

#[test]
fn test_impl_4(){
    let mut f = Foo { x: 0 };
    f.foo_ret(10);
}

#[test]
#[should_panic]
fn test_impl_5(){
    let mut f = Foo { x: 0 };
    f.foo_ret(16);
}
