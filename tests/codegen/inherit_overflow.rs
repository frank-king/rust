// compile-flags: -Zmir-enable-passes=+Inline,+ConstProp --crate-type lib
// revisions: ASSERT NOASSERT
//[ASSERT] compile-flags: -Coverflow-checks=on
//[NOASSERT] compile-flags: -Coverflow-checks=off

// CHECK-LABEL: define{{.*}} @assertion
// ASSERT: call void @_ZN4core9panicking5panic17h
// NOASSERT: ret i8 0
#[no_mangle]
pub fn assertion() -> u8 {
    // Optimized MIR will replace this `CheckedBinaryOp` by `const (0, true)`.
    // Verify that codegen does or does not emit the panic.
    <u8 as std::ops::Add>::add(255, 1)
}
