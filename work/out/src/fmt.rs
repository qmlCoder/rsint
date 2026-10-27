extern "C" {
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn erf(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn erfc(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub const SML_FLOAT64: ::core::ffi::c_double = DBL_EPSILON * 0.5f64;
pub const SML_FLOAT80: ::core::ffi::c_double = 2.0e-20f64;
pub const SQRTPIE4: ::core::ffi::c_double = 0.8862269254527580136490837416705725913987747280611935641069038949264f64;
pub const ERFC_bound: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
static mut TURNOVER_POINT: [::core::ffi::c_double; 40] = [
    0.0f64,
    0.0f64,
    0.866025403784f64,
    1.295010032056f64,
    1.705493613097f64,
    2.106432965305f64,
    2.501471934009f64,
    2.892473348218f64,
    3.280525047072f64,
    3.666320693281f64,
    4.05033123037f64,
    4.432891808508f64,
    4.814249856864f64,
    5.194593501454f64,
    5.574069276051f64,
    5.952793645111f64,
    6.330860773135f64,
    6.708347923415f64,
    7.08531930745f64,
    7.461828891625f64,
    7.837922483937f64,
    8.213639312398f64,
    8.589013237349f64,
    8.964073695432f64,
    9.338846443746f64,
    9.713354153046f64,
    10.08761688545f64,
    10.46165248270f64,
    10.83547688448f64,
    11.20910439128f64,
    11.58254788331f64,
    11.95581900374f64,
    12.32892831326f64,
    12.70188542111f64,
    13.07469909673f64,
    13.44737736550f64,
    13.81992759110f64,
    14.19235654675f64,
    14.56467047710f64,
    14.93687515212f64,
];
unsafe extern "C" fn fmt1_gamma_inc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_double = m as ::core::ffi::c_double + 0.5f64;
    let mut bi: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.5f64 * exp(-t);
    let mut x: ::core::ffi::c_double = e;
    let mut s: ::core::ffi::c_double = e;
    let mut tol: ::core::ffi::c_double = SML_FLOAT64 * e;
    bi = b + 1.0f64;
    while x > tol {
        x *= t / bi;
        s += x;
        bi += 1.0f64;
    }
    *f.offset(m as isize) = s / b;
    i = m;
    while i > 0 as ::core::ffi::c_int {
        b -= 1.0f64;
        *f.offset((i - 1 as ::core::ffi::c_int) as isize) = (e
            + t * *f.offset(i as isize)) / b;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gamma_inc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    if t == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut i: ::core::ffi::c_int = 0;
        *f.offset(0 as ::core::ffi::c_int as isize) = 1.0f64;
        i = 1 as ::core::ffi::c_int;
        while i <= m {
            *f.offset(i as isize) = 1.0f64
                / (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            i += 1;
        }
    } else if t < TURNOVER_POINT[m as usize] {
        fmt1_gamma_inc_like(f, t, m);
    } else {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut tt: ::core::ffi::c_double = sqrt(t);
        *f.offset(0 as ::core::ffi::c_int as isize) = SQRTPIE4 / tt * erf(tt);
        let mut e: ::core::ffi::c_double = exp(-t);
        let mut b: ::core::ffi::c_double = 0.5f64 / t;
        i_0 = 1 as ::core::ffi::c_int;
        while i_0 <= m {
            *f.offset(i_0 as isize) = b
                * ((2 as ::core::ffi::c_int * i_0 - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                    * *f.offset((i_0 - 1 as ::core::ffi::c_int) as isize) - e);
            i_0 += 1;
        }
    };
}
unsafe extern "C" fn fmt1_lgamma_inc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    let mut b: ::core::ffi::c_double = m as ::core::ffi::c_double + 0.5f64;
    let mut bi: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.5f64 * exp(-t);
    let mut x: ::core::ffi::c_double = e;
    let mut s: ::core::ffi::c_double = e;
    let mut tol: ::core::ffi::c_double = SML_FLOAT80 * e;
    let mut i: ::core::ffi::c_int = 0;
    bi = b + 1.0f64;
    while x > tol {
        x *= t / bi;
        s += x;
        bi += 1.0f64;
    }
    *f.offset(m as isize) = s / b;
    i = m;
    while i > 0 as ::core::ffi::c_int {
        b -= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *f.offset((i - 1 as ::core::ffi::c_int) as isize) = (e
            + t * *f.offset(i as isize)) / b;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lgamma_inc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    if t == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut i: ::core::ffi::c_int = 0;
        *f.offset(0 as ::core::ffi::c_int as isize) = 1.0f64;
        i = 1 as ::core::ffi::c_int;
        while i <= m {
            *f.offset(i as isize) = 1.0f64
                / (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            i += 1;
        }
    } else if t < TURNOVER_POINT[m as usize] {
        fmt1_lgamma_inc_like(f, t, m);
    } else {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut tt: ::core::ffi::c_double = sqrt(t);
        *f.offset(0 as ::core::ffi::c_int as isize) = SQRTPIE4 / tt * erf(tt);
        let mut e: ::core::ffi::c_double = exp(-t);
        let mut b: ::core::ffi::c_double = 0.5f64 / t;
        i_0 = 1 as ::core::ffi::c_int;
        while i_0 <= m {
            *f.offset(i_0 as isize) = b
                * ((2 as ::core::ffi::c_int * i_0 - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                    * *f.offset((i_0 - 1 as ::core::ffi::c_int) as isize) - e);
            i_0 += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn pow_(
    mut base: ::core::ffi::c_double,
    mut exponent: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    i = 1 as ::core::ffi::c_int;
    while i <= exponent {
        if i & exponent != 0 {
            result *= base;
        }
        base *= base;
        i <<= 1 as ::core::ffi::c_int;
    }
    return result;
}
#[inline]
unsafe extern "C" fn powl_(
    mut base: ::core::ffi::c_double,
    mut exponent: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_double = 1.0f64;
    i = 1 as ::core::ffi::c_int;
    while i <= exponent {
        if i & exponent != 0 {
            result *= base;
        }
        base *= base;
        i <<= 1 as ::core::ffi::c_int;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn fmt1_erfc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut lower: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lower2: ::core::ffi::c_double = lower * lower;
    let mut b: ::core::ffi::c_double = m as ::core::ffi::c_double + 0.5f64;
    let mut bi: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.5f64 * exp(-t);
    let mut e1: ::core::ffi::c_double = 0.5f64 * exp(-t * lower2) * lower;
    e1 *= pow_(lower2, m);
    let mut x: ::core::ffi::c_double = e;
    let mut x1: ::core::ffi::c_double = e1;
    let mut s: ::core::ffi::c_double = e - e1;
    let mut div: ::core::ffi::c_double = 1.0f64;
    let mut delta: ::core::ffi::c_double = s;
    let mut tol: ::core::ffi::c_double = SML_FLOAT64 * fabs(delta);
    bi = b + 1.0f64;
    while fabs(delta) > tol {
        div *= t / bi;
        x1 *= lower2;
        delta = (x - x1) * div;
        s += delta;
        bi += 1.0f64;
    }
    let mut val: ::core::ffi::c_double = s / b;
    *f.offset(m as isize) = val;
    i = m;
    while i > 0 as ::core::ffi::c_int {
        b -= 1.0f64;
        e1 /= lower2;
        val = (e - e1 + t * val) / b;
        *f.offset((i - 1 as ::core::ffi::c_int) as isize) = val;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fmt_erfc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut lower: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    if lower == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return gamma_inc_like(f, t, m);
    }
    let mut i: ::core::ffi::c_int = 0;
    let mut lower2: ::core::ffi::c_double = lower * lower;
    if t * lower2 > ERFC_bound as ::core::ffi::c_double {
        i = 0 as ::core::ffi::c_int;
        while i <= m {
            *f.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        return;
    }
    if t == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut e1: ::core::ffi::c_double = lower;
        *f.offset(0 as ::core::ffi::c_int as isize) = 1.0f64 - e1;
        i = 1 as ::core::ffi::c_int;
        while i <= m {
            e1 *= lower2;
            *f.offset(i as isize) = (1.0f64 - e1)
                / (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            i += 1;
        }
    } else if t < TURNOVER_POINT[m as usize] {
        fmt1_erfc_like(f, t, lower, m);
    } else {
        let mut tt: ::core::ffi::c_double = sqrt(t);
        let mut val: ::core::ffi::c_double = SQRTPIE4 / tt
            * (erfc(lower * tt) - erfc(tt));
        *f.offset(0 as ::core::ffi::c_int as isize) = val;
        let mut e: ::core::ffi::c_double = exp(-t);
        let mut e1_0: ::core::ffi::c_double = exp(-t * lower2) * lower;
        let mut b: ::core::ffi::c_double = 0.5f64 / t;
        i = 0 as ::core::ffi::c_int;
        while i < m {
            val = b
                * ((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double * val - e + e1_0);
            e1_0 *= lower2;
            *f.offset((i + 1 as ::core::ffi::c_int) as isize) = val;
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fmt_lerfc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut lower: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    if lower == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return lgamma_inc_like(f, t, m);
    }
    let mut i: ::core::ffi::c_int = 0;
    let mut lower2: ::core::ffi::c_double = lower * lower;
    if t * lower2 > ERFC_bound as ::core::ffi::c_double {
        i = 0 as ::core::ffi::c_int;
        while i <= m {
            *f.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        return;
    }
    if t == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut e1: ::core::ffi::c_double = lower;
        *f.offset(0 as ::core::ffi::c_int as isize) = 1.0f64 - e1;
        i = 1 as ::core::ffi::c_int;
        while i <= m {
            e1 *= lower2;
            *f.offset(i as isize) = (1.0f64 - e1)
                / (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            i += 1;
        }
    } else if t < TURNOVER_POINT[m as usize] {
        fmt1_lerfc_like(f, t, lower, m);
    } else {
        let mut tt: ::core::ffi::c_double = sqrt(t);
        let mut val: ::core::ffi::c_double = SQRTPIE4 / tt
            * (erfc(lower * tt) - erfc(tt));
        *f.offset(0 as ::core::ffi::c_int as isize) = val;
        let mut e: ::core::ffi::c_double = exp(-t);
        let mut e1_0: ::core::ffi::c_double = exp(-t * lower2) * lower;
        let mut b: ::core::ffi::c_double = 0.5f64 / t;
        i = 0 as ::core::ffi::c_int;
        while i < m {
            val = b
                * ((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_double * val - e + e1_0);
            e1_0 *= lower2;
            *f.offset((i + 1 as ::core::ffi::c_int) as isize) = val;
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fmt1_lerfc_like(
    mut f: *mut ::core::ffi::c_double,
    mut t: ::core::ffi::c_double,
    mut lower: ::core::ffi::c_double,
    mut m: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lower2: ::core::ffi::c_double = lower * lower;
    let mut b: ::core::ffi::c_double = m as ::core::ffi::c_double + 0.5f64;
    let mut bi: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.5f64 * exp(-t);
    let mut e1: ::core::ffi::c_double = 0.5f64 * exp(-t * lower2) * lower;
    e1 *= powl_(lower2, m);
    let mut x: ::core::ffi::c_double = e;
    let mut x1: ::core::ffi::c_double = e1;
    let mut s: ::core::ffi::c_double = e - e1;
    let mut div: ::core::ffi::c_double = 1.0f64;
    let mut delta: ::core::ffi::c_double = s;
    let mut tol: ::core::ffi::c_double = SML_FLOAT80 * fabs(delta);
    bi = b + 1.0f64;
    while fabs(delta) > tol {
        div *= t / bi;
        x1 *= lower2;
        delta = (x - x1) * div;
        s += delta;
        bi += 1.0f64;
    }
    let mut val: ::core::ffi::c_double = s / b;
    *f.offset(m as isize) = val;
    i = m;
    while i > 0 as ::core::ffi::c_int {
        b -= 1.0f64;
        e1 /= lower2;
        val = (e - e1 + t * val) / b;
        *f.offset((i - 1 as ::core::ffi::c_int) as isize) = val;
        i -= 1;
    }
}
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
