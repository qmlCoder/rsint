# rsint

Mechanical C→Rust transliteration of [libcint](https://github.com/sunqm/libcint)
(v6.1.3), generated with [c2rust](https://github.com/immunant/c2rust) 0.22.1.

libcint is an open-source library for analytical Gaussian integrals. It provides
a C/Fortran API to evaluate one-electron / two-electron integrals for Cartesian,
real-spherical, and spinor Gaussian type functions.

This crate is **not an idiomatic Rust API**. It is the raw c2rust output: lots of
`unsafe`, raw pointers, mutable globals, and `extern "C"` functions, kept close to
the C so that results can be verified numerically against the original C library.

## Status

- 41 C source files → ~270k lines of Rust (`src/`).
- Builds as `rlib` + `cdylib` (`librsint.so`).
- Core real-valued integrals verified **bit-identical** (max abs error `0.000e+00`)
  against the C library:
  `cint1e_ovlp_{cart,sph}`, `cint1e_kin_{cart,sph}`, `cint1e_nuc_{cart,sph}`,
  `cint2e_{cart,sph}`, `cint2c2e_{cart,sph}`, `cint3c2e_{cart,sph}`.

## Requirements

This crate requires the exact pinned nightly toolchain (the transliteration uses
the `extern_types`, `label_break_value`, and `raw_ref_op` unstable features):

```text
nightly-2025-02-01
```

It will **not** build on stable or on newer nightlies (c2rust output depends on
feature/f128 behavior that has since changed).

## Usage

The library exposes `#[no_mangle] pub unsafe extern "C" fn` symbols (the libcint
C API) rather than a safe Rust wrapper. Use it like the C library, e.g. via FFI:

```rust
use std::ffi::c_int;

extern "C" {
    fn cint1e_ovlp_cart(
        buf: *mut f64,
        shls: *const c_int,
        atm: *const c_int,
        natm: c_int,
        bas: *const c_int,
        nbas: c_int,
        env: *const f64,
        cintopt: *const core::ffi::c_void,
    ) -> c_int;
}
```

Or link against the `cdylib` from another language exactly as you would link the
original `libcint`.

## Known limitations

- **Complex/spinor functions are missing.** c2rust 0.22.1 cannot translate C's
  `double complex` / `_Complex` types, so 479 complex/spinor functions were
  skipped.
- **`misc.c` is not translated** (it uses `_Complex_I`). The one symbol needed by
  the real path, `CINTsquare_dist`, is provided in `helpers.rs`.
- `long double` / `__float128` / quadmath were replaced with `double` in the
  translation input, so quad precision is reduced to double precision.

## License

Apache-2.0 (same as upstream libcint). See [LICENSE](LICENSE). Upstream:
<https://github.com/sunqm/libcint>.
