// misc.c 中被转译产物引用但未定义的符号, 手动补充
#[no_mangle]
pub unsafe extern "C" fn CINTsquare_dist(
    r1: *const ::core::ffi::c_double,
    r2: *const ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let dx = *r1.offset(0) - *r2.offset(0);
    let dy = *r1.offset(1) - *r2.offset(1);
    let dz = *r1.offset(2) - *r2.offset(2);
    dx * dx + dy * dy + dz * dz
}
