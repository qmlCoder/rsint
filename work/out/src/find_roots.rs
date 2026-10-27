extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe extern "C" fn R_dnode(
    mut a: *mut ::core::ffi::c_double,
    mut roots: *mut ::core::ffi::c_double,
    mut order: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let accrt: ::core::ffi::c_double = 1e-15f64;
    let mut x0: ::core::ffi::c_double = 0.;
    let mut x1: ::core::ffi::c_double = 0.;
    let mut xi: ::core::ffi::c_double = 0.;
    let mut x1init: ::core::ffi::c_double = 0.;
    let mut p0: ::core::ffi::c_double = 0.;
    let mut p1: ::core::ffi::c_double = 0.;
    let mut pi: ::core::ffi::c_double = 0.;
    let mut p1init: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    x1init = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    p1init = *a.offset(0 as ::core::ffi::c_int as isize);
    m = 0 as ::core::ffi::c_int;
    while m < order {
        x0 = x1init;
        p0 = p1init;
        x1init = *roots.offset(m as isize);
        p1init = *a.offset(order as isize);
        i = 1 as ::core::ffi::c_int;
        while i <= order {
            p1init = p1init * x1init + *a.offset((order - i) as isize);
            i += 1;
        }
        if !(p1init == 0 as ::core::ffi::c_int as ::core::ffi::c_double) {
            if p0 * p1init > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                fprintf(
                    stderr,
                    b"ROOT NUMBER %d WAS NOT FOUND FOR POLYNOMIAL OF ORDER %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    m,
                    order,
                );
                return 1 as ::core::ffi::c_int;
            }
            if x0 <= x1init {
                x1 = x1init;
                p1 = p1init;
            } else {
                x1 = x0;
                p1 = p0;
                x0 = x1init;
                p0 = p1init;
            }
            if p1 == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *roots.offset(m as isize) = x1;
            } else if p0 == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *roots.offset(m as isize) = x0;
            } else {
                xi = x0 + (x0 - x1) / (p1 - p0) * p0;
                n = 0 as ::core::ffi::c_int;
                while fabs(x1 - x0) > x1 * accrt {
                    n += 1;
                    if n > 200 as ::core::ffi::c_int {
                        fprintf(
                            stderr,
                            b"libcint::rys_roots NO CONV. IN R_dnode\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return 1 as ::core::ffi::c_int;
                    }
                    pi = *a.offset(order as isize);
                    i = 1 as ::core::ffi::c_int;
                    while i <= order {
                        pi = pi * xi + *a.offset((order - i) as isize);
                        i += 1;
                    }
                    if pi == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        break;
                    }
                    if p0 * pi <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        x1 = xi;
                        p1 = pi;
                        xi = x0 * 0.25f64 + xi * 0.75f64;
                    } else {
                        x0 = xi;
                        p0 = pi;
                        xi = xi * 0.75f64 + x1 * 0.25f64;
                    }
                    pi = *a.offset(order as isize);
                    i = 1 as ::core::ffi::c_int;
                    while i <= order {
                        pi = pi * xi + *a.offset((order - i) as isize);
                        i += 1;
                    }
                    if pi == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        break;
                    }
                    if p0 * pi <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        x1 = xi;
                        p1 = pi;
                    } else {
                        x0 = xi;
                        p0 = pi;
                    }
                    xi = x0 + (x0 - x1) / (p1 - p0) * p0;
                }
                *roots.offset(m as isize) = xi;
            }
        }
        m += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _qr_step(
    mut A: *mut ::core::ffi::c_double,
    mut nroots: ::core::ffi::c_int,
    mut n0: ::core::ffi::c_int,
    mut n1: ::core::ffi::c_int,
    mut shift: ::core::ffi::c_double,
) {
    let mut m1: ::core::ffi::c_int = n0 + 1 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m3: ::core::ffi::c_int = 0;
    let mut j1: ::core::ffi::c_int = 0;
    let mut j2: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_double = *A.offset((n0 * nroots + n0) as isize) - shift;
    let mut s: ::core::ffi::c_double = *A.offset((m1 * nroots + n0) as isize);
    let mut v: ::core::ffi::c_double = sqrt(c * c + s * s);
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if v == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        v = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        c = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        s = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    v = 1.0f64 / v;
    c *= v;
    s *= v;
    k = n0;
    while k < nroots {
        x = *A.offset((n0 * nroots + k) as isize);
        y = *A.offset((m1 * nroots + k) as isize);
        *A.offset((n0 * nroots + k) as isize) = c * x + s * y;
        *A.offset((m1 * nroots + k) as isize) = c * y - s * x;
        k += 1;
    }
    m3 = if n1 < n0 + 3 as ::core::ffi::c_int {
        n1
    } else {
        n0 + 3 as ::core::ffi::c_int
    };
    k = 0 as ::core::ffi::c_int;
    while k < m3 {
        x = *A.offset((k * nroots + n0) as isize);
        y = *A.offset((k * nroots + m1) as isize);
        *A.offset((k * nroots + n0) as isize) = c * x + s * y;
        *A.offset((k * nroots + m1) as isize) = c * y - s * x;
        k += 1;
    }
    j = n0;
    while j < n1 - 2 as ::core::ffi::c_int {
        j1 = j + 1 as ::core::ffi::c_int;
        j2 = j + 2 as ::core::ffi::c_int;
        c = *A.offset((j1 * nroots + j) as isize);
        s = *A.offset((j2 * nroots + j) as isize);
        v = sqrt(c * c + s * s);
        *A.offset((j1 * nroots + j) as isize) = v;
        *A.offset((j2 * nroots + j) as isize) = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        if v == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            v = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            c = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            s = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        v = 1.0f64 / v;
        c *= v;
        s *= v;
        k = j1;
        while k < nroots {
            x = *A.offset((j1 * nroots + k) as isize);
            y = *A.offset((j2 * nroots + k) as isize);
            *A.offset((j1 * nroots + k) as isize) = c * x + s * y;
            *A.offset((j2 * nroots + k) as isize) = c * y - s * x;
            k += 1;
        }
        m3 = if n1 < j + 4 as ::core::ffi::c_int {
            n1
        } else {
            j + 4 as ::core::ffi::c_int
        };
        k = 0 as ::core::ffi::c_int;
        while k < m3 {
            x = *A.offset((k * nroots + j1) as isize);
            y = *A.offset((k * nroots + j2) as isize);
            *A.offset((k * nroots + j1) as isize) = c * x + s * y;
            *A.offset((k * nroots + j2) as isize) = c * y - s * x;
            k += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn _hessenberg_qr(
    mut A: *mut ::core::ffi::c_double,
    mut nroots: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut eps: ::core::ffi::c_double = 1e-15f64;
    let mut maxits: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut n0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n1: ::core::ffi::c_int = nroots;
    let mut its: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int = 0;
    let mut ic: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 0;
    ic = 0 as ::core::ffi::c_int;
    while ic < nroots * maxits {
        k = n0;
        while (k + 1 as ::core::ffi::c_int) < n1 {
            let mut s: ::core::ffi::c_double = fabs(*A.offset((k * nroots + k) as isize))
                + fabs(
                    *A
                        .offset(
                            ((k + 1 as ::core::ffi::c_int) * nroots + k
                                + 1 as ::core::ffi::c_int) as isize,
                        ),
                );
            if fabs(*A.offset(((k + 1 as ::core::ffi::c_int) * nroots + k) as isize))
                < eps * s
            {
                break;
            }
            k += 1 as ::core::ffi::c_int;
        }
        k1 = k + 1 as ::core::ffi::c_int;
        if k1 < n1 {
            *A.offset((k1 * nroots + k) as isize) = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            n0 = k1;
            its = 0 as ::core::ffi::c_int;
            if n0 + 1 as ::core::ffi::c_int >= n1 {
                n0 = 0 as ::core::ffi::c_int;
                n1 = k1;
                if n1 < 2 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            }
        } else {
            let mut m1: ::core::ffi::c_int = n1 - 1 as ::core::ffi::c_int;
            let mut m2: ::core::ffi::c_int = n1 - 2 as ::core::ffi::c_int;
            let mut a11: ::core::ffi::c_double = *A.offset((m1 * nroots + m1) as isize);
            let mut a22: ::core::ffi::c_double = *A.offset((m2 * nroots + m2) as isize);
            let mut shift: ::core::ffi::c_double = 0.;
            let mut t: ::core::ffi::c_double = a11 + a22;
            let mut s_0: ::core::ffi::c_double = (a11 - a22) * (a11 - a22);
            s_0
                += 4 as ::core::ffi::c_int as ::core::ffi::c_double
                    * *A.offset((m1 * nroots + m2) as isize)
                    * *A.offset((m2 * nroots + m1) as isize);
            if s_0 > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                s_0 = sqrt(s_0);
                let mut a: ::core::ffi::c_double = (t + s_0) * 0.5f64;
                let mut b: ::core::ffi::c_double = (t - s_0) * 0.5f64;
                if fabs(a11 - a) > fabs(a11 - b) {
                    shift = b;
                } else {
                    shift = a;
                }
            } else {
                if n1 == 2 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"hessenberg_qr: failed to find real roots\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as ::core::ffi::c_int;
                }
                shift = t * 0.5f64;
            }
            its += 1 as ::core::ffi::c_int;
            _qr_step(A, nroots, n0, n1, shift);
            if its > maxits {
                fprintf(
                    stderr,
                    b"hessenberg_qr: failed to converge after %d steps\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    its,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        ic += 1;
    }
    fprintf(
        stderr,
        b"hessenberg_qr failed\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _CINT_polynomial_roots(
    mut roots: *mut ::core::ffi::c_double,
    mut cs: *mut ::core::ffi::c_double,
    mut nroots: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nroots == 1 as ::core::ffi::c_int {
        *roots.offset(0 as ::core::ffi::c_int as isize) = -*cs
            .offset(2 as ::core::ffi::c_int as isize)
            / *cs.offset(3 as ::core::ffi::c_int as isize);
        return 0 as ::core::ffi::c_int;
    } else if nroots == 2 as ::core::ffi::c_int {
        let mut dum: ::core::ffi::c_double = sqrt(
            *cs
                .offset(
                    (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as isize,
                )
                * *cs
                    .offset(
                        (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    )
                - 4 as ::core::ffi::c_int as ::core::ffi::c_double
                    * *cs
                        .offset(
                            (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int) as isize,
                        )
                    * *cs
                        .offset(
                            (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int) as isize,
                        ),
        );
        *roots.offset(0 as ::core::ffi::c_int as isize) = (-*cs
            .offset(
                (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) - dum)
            / *cs
                .offset(
                    (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as isize,
                ) / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        *roots.offset(1 as ::core::ffi::c_int as isize) = (-*cs
            .offset(
                (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) + dum)
            / *cs
                .offset(
                    (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as isize,
                ) / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        return 0 as ::core::ffi::c_int;
    }
    let mut A: [::core::ffi::c_double; 1024] = [0.; 1024];
    let mut nroots1: ::core::ffi::c_int = nroots + 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut fac: ::core::ffi::c_double = -1.0f64
        / *cs.offset((nroots * nroots1 + nroots) as isize);
    i = 0 as ::core::ffi::c_int;
    while i < nroots {
        A[(nroots - 1 as ::core::ffi::c_int - i) as usize] = *cs
            .offset((nroots * nroots1 + i) as isize) * fac;
        i += 1;
    }
    i = nroots;
    while i < nroots * nroots {
        A[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nroots - 1 as ::core::ffi::c_int {
        A[((i + 1 as ::core::ffi::c_int) * nroots + i) as usize] = 1.0f64;
        i += 1;
    }
    let mut err: ::core::ffi::c_int = _hessenberg_qr(
        &raw mut A as *mut ::core::ffi::c_double,
        nroots,
    );
    if err == 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < nroots {
            *roots.offset((nroots - 1 as ::core::ffi::c_int - i) as isize) = A[(i
                * nroots + i) as usize];
            i += 1;
        }
    } else {
        let mut k: ::core::ffi::c_int = 0;
        let mut order: ::core::ffi::c_int = 0;
        let mut a: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
            ::core::ffi::c_double,
        >();
        let mut dum_0: ::core::ffi::c_double = sqrt(
            *cs
                .offset(
                    (2 as ::core::ffi::c_int * nroots1 + 1 as ::core::ffi::c_int)
                        as isize,
                )
                * *cs
                    .offset(
                        (2 as ::core::ffi::c_int * nroots1 + 1 as ::core::ffi::c_int)
                            as isize,
                    )
                - 4 as ::core::ffi::c_int as ::core::ffi::c_double
                    * *cs
                        .offset(
                            (2 as ::core::ffi::c_int * nroots1 + 0 as ::core::ffi::c_int)
                                as isize,
                        )
                    * *cs
                        .offset(
                            (2 as ::core::ffi::c_int * nroots1 + 2 as ::core::ffi::c_int)
                                as isize,
                        ),
        );
        *roots.offset(0 as ::core::ffi::c_int as isize) = 0.5f64
            * (-*cs
                .offset(
                    (2 as ::core::ffi::c_int * nroots1 + 1 as ::core::ffi::c_int)
                        as isize,
                ) - dum_0)
            / *cs
                .offset(
                    (2 as ::core::ffi::c_int * nroots1 + 2 as ::core::ffi::c_int)
                        as isize,
                );
        *roots.offset(1 as ::core::ffi::c_int as isize) = 0.5f64
            * (-*cs
                .offset(
                    (2 as ::core::ffi::c_int * nroots1 + 1 as ::core::ffi::c_int)
                        as isize,
                ) + dum_0)
            / *cs
                .offset(
                    (2 as ::core::ffi::c_int * nroots1 + 2 as ::core::ffi::c_int)
                        as isize,
                );
        i = 2 as ::core::ffi::c_int;
        while i < nroots {
            *roots.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        k = 2 as ::core::ffi::c_int;
        while k < nroots {
            order = k + 1 as ::core::ffi::c_int;
            a = cs.offset((order * nroots1) as isize);
            err = R_dnode(a, roots, order);
            if err != 0 {
                break;
            }
            k += 1;
        }
    }
    return err;
}
