#[no_mangle]
pub unsafe extern "C" fn CINTdset0(
    mut n: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *x.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdaxpy2v(
    mut n: ::core::ffi::c_int,
    mut a: ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
    mut v: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *v.offset(i as isize) = a * *x.offset(i as isize) + *y.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdmat_transpose(
    mut a_t: *mut ::core::ffi::c_double,
    mut a: *mut ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < n - 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < m {
            *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize) = *a
                .offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize) = *a
                .offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 2 as ::core::ffi::c_int) * m + i) as isize) = *a
                .offset((i * n + j + 2 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 3 as ::core::ffi::c_int) * m + i) as isize) = *a
                .offset((i * n + j + 3 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        j += 4 as ::core::ffi::c_int;
    }
    match n - j {
        1 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset((j * m + i) as isize) = *a.offset((i * n + j) as isize);
                i += 1;
            }
        }
        2 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize) = *a
                    .offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize) = *a
                    .offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
        }
        3 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize) = *a
                    .offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize) = *a
                    .offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 2 as ::core::ffi::c_int) * m + i) as isize) = *a
                    .offset((i * n + j + 2 as ::core::ffi::c_int) as isize);
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTdplus_transpose(
    mut a_t: *mut ::core::ffi::c_double,
    mut a: *mut ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < n - 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < m {
            *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize)
                += *a.offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize)
                += *a.offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 2 as ::core::ffi::c_int) * m + i) as isize)
                += *a.offset((i * n + j + 2 as ::core::ffi::c_int) as isize);
            *a_t.offset(((j + 3 as ::core::ffi::c_int) * m + i) as isize)
                += *a.offset((i * n + j + 3 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        j += 4 as ::core::ffi::c_int;
    }
    match n - j {
        1 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset((j * m + i) as isize) += *a.offset((i * n + j) as isize);
                i += 1;
            }
        }
        2 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize)
                    += *a.offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize)
                    += *a.offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
        }
        3 => {
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *a_t.offset(((j + 0 as ::core::ffi::c_int) * m + i) as isize)
                    += *a.offset((i * n + j + 0 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 1 as ::core::ffi::c_int) * m + i) as isize)
                    += *a.offset((i * n + j + 1 as ::core::ffi::c_int) as isize);
                *a_t.offset(((j + 2 as ::core::ffi::c_int) * m + i) as isize)
                    += *a.offset((i * n + j + 2 as ::core::ffi::c_int) as isize);
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NN1(
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut c: *mut ::core::ffi::c_double,
    mut ldc: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut bi: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < n {
        i = 0 as ::core::ffi::c_int;
        while i < m {
            *c.offset((i + ldc * j) as isize) = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            i += 1;
        }
        kp = 0 as ::core::ffi::c_int;
        while kp < k {
            bi = *b.offset((kp + k * j) as isize);
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *c.offset((i + ldc * j) as isize)
                    += *a.offset((i + m * kp) as isize) * bi;
                i += 1;
            }
            kp += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NN(
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut c: *mut ::core::ffi::c_double,
) {
    CINTdgemm_NN1(m, n, k, a, b, c, m);
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_TN(
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut c: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < n {
        i = 0 as ::core::ffi::c_int;
        while i < m {
            ci = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            kp = 0 as ::core::ffi::c_int;
            while kp < k {
                ci
                    += *a.offset((kp + k * i) as isize)
                        * *b.offset((kp + k * j) as isize);
                kp += 1;
            }
            *c.offset((i + m * j) as isize) = ci;
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NT(
    mut m: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut a: *mut ::core::ffi::c_double,
    mut b: *mut ::core::ffi::c_double,
    mut c: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut bi: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < n {
        i = 0 as ::core::ffi::c_int;
        while i < m {
            *c.offset((i + m * j) as isize) = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            i += 1;
        }
        kp = 0 as ::core::ffi::c_int;
        while kp < k {
            bi = *b.offset((j + n * kp) as isize);
            i = 0 as ::core::ffi::c_int;
            while i < m {
                *c.offset((i + m * j) as isize) += *a.offset((i + m * kp) as isize) * bi;
                i += 1;
            }
            kp += 1;
        }
        j += 1;
    }
}
