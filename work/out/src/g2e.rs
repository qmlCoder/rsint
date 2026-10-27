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
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn CINTcart_comp(
        nx: *mut ::core::ffi::c_int,
        ny: *mut ::core::ffi::c_int,
        nz: *mut ::core::ffi::c_int,
        lmax: ::core::ffi::c_int,
    );
    fn CINTrys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        u: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn CINTsr_rys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        lower: ::core::ffi::c_double,
        u: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn CINTcommon_fac_sp(l: ::core::ffi::c_int) -> ::core::ffi::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [::core::ffi::c_double; 3],
    pub eij: ::core::ffi::c_double,
    pub cceij: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut ::core::ffi::c_int,
    pub non0ctr: *mut *mut ::core::ffi::c_int,
    pub sortedidx: *mut *mut ::core::ffi::c_int,
    pub nbas: ::core::ffi::c_int,
    pub log_max_coeff: *mut *mut ::core::ffi::c_double,
    pub pairdata: *mut *mut PairData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTEnvVars {
    pub atm: *mut ::core::ffi::c_int,
    pub bas: *mut ::core::ffi::c_int,
    pub env: *mut ::core::ffi::c_double,
    pub shls: *mut ::core::ffi::c_int,
    pub natm: ::core::ffi::c_int,
    pub nbas: ::core::ffi::c_int,
    pub i_l: ::core::ffi::c_int,
    pub j_l: ::core::ffi::c_int,
    pub k_l: ::core::ffi::c_int,
    pub l_l: ::core::ffi::c_int,
    pub nfi: ::core::ffi::c_int,
    pub nfj: ::core::ffi::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub nf: ::core::ffi::c_int,
    pub rys_order: ::core::ffi::c_int,
    pub x_ctr: [::core::ffi::c_int; 4],
    pub gbits: ::core::ffi::c_int,
    pub ncomp_e1: ::core::ffi::c_int,
    pub ncomp_e2: ::core::ffi::c_int,
    pub ncomp_tensor: ::core::ffi::c_int,
    pub li_ceil: ::core::ffi::c_int,
    pub lj_ceil: ::core::ffi::c_int,
    pub lk_ceil: ::core::ffi::c_int,
    pub ll_ceil: ::core::ffi::c_int,
    pub g_stride_i: ::core::ffi::c_int,
    pub g_stride_k: ::core::ffi::c_int,
    pub g_stride_l: ::core::ffi::c_int,
    pub g_stride_j: ::core::ffi::c_int,
    pub nrys_roots: ::core::ffi::c_int,
    pub g_size: ::core::ffi::c_int,
    pub g2d_ijmax: ::core::ffi::c_int,
    pub g2d_klmax: ::core::ffi::c_int,
    pub common_factor: ::core::ffi::c_double,
    pub expcutoff: ::core::ffi::c_double,
    pub rirj: [::core::ffi::c_double; 3],
    pub rkrl: [::core::ffi::c_double; 3],
    pub rx_in_rijrx: *mut ::core::ffi::c_double,
    pub rx_in_rklrx: *mut ::core::ffi::c_double,
    pub ri: *mut ::core::ffi::c_double,
    pub rj: *mut ::core::ffi::c_double,
    pub rk: *mut ::core::ffi::c_double,
    pub c2rust_unnamed_1: C2RustUnnamed,
    pub f_g0_2e: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub f_g0_2d4d: Option<unsafe extern "C" fn() -> ()>,
    pub f_gout: Option<unsafe extern "C" fn() -> ()>,
    pub opt: *mut CINTOpt,
    pub idx: *mut ::core::ffi::c_int,
    pub ai: [::core::ffi::c_double; 1],
    pub aj: [::core::ffi::c_double; 1],
    pub ak: [::core::ffi::c_double; 1],
    pub al: [::core::ffi::c_double; 1],
    pub fac: [::core::ffi::c_double; 1],
    pub rij: [::core::ffi::c_double; 3],
    pub rkl: [::core::ffi::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rl: *mut ::core::ffi::c_double,
    pub grids: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nfl: ::core::ffi::c_int,
    pub ngrids: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nfk: ::core::ffi::c_int,
    pub grids_offset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rys2eT {
    pub c00x: [::core::ffi::c_double; 32],
    pub c00y: [::core::ffi::c_double; 32],
    pub c00z: [::core::ffi::c_double; 32],
    pub c0px: [::core::ffi::c_double; 32],
    pub c0py: [::core::ffi::c_double; 32],
    pub c0pz: [::core::ffi::c_double; 32],
    pub b01: [::core::ffi::c_double; 32],
    pub b00: [::core::ffi::c_double; 32],
    pub b10: [::core::ffi::c_double; 32],
}
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 91] = unsafe {
    ::core::mem::transmute::<
        [u8; 91],
        [::core::ffi::c_char; 91],
    >(
        *b"void CINTinit_int2e_EnvVars(CINTEnvVars *, int *, int *, int *, int, int *, int, double *)\0",
    )
};
pub const PTR_EXPCUTOFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTR_RANGE_OMEGA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ATM_SLOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EXPCUTOFF_SR: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: ::core::ffi::c_int = *shls.offset(0 as ::core::ffi::c_int as isize);
    let j_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    let k_sh: ::core::ffi::c_int = *shls.offset(2 as ::core::ffi::c_int as isize);
    let l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    (*envs).i_l = *bas.offset((BAS_SLOTS * i_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).j_l = *bas.offset((BAS_SLOTS * j_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).k_l = *bas.offset((BAS_SLOTS * k_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).l_l = *bas.offset((BAS_SLOTS * l_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[0 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * i_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[1 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * j_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[2 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * k_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[3 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * l_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).nfi = ((*envs).i_l + 1 as ::core::ffi::c_int)
        * ((*envs).i_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).nfj = ((*envs).j_l + 1 as ::core::ffi::c_int)
        * ((*envs).j_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).c2rust_unnamed.nfk = ((*envs).k_l + 1 as ::core::ffi::c_int)
        * ((*envs).k_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).c2rust_unnamed_0.nfl = ((*envs).l_l + 1 as ::core::ffi::c_int)
        * ((*envs).l_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk * (*envs).c2rust_unnamed_0.nfl
        * (*envs).nfj;
    (*envs).ri = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).rj = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).rk = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * k_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).c2rust_unnamed_1.rl = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * l_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).common_factor = M_PI * M_PI * M_PI
        * 2 as ::core::ffi::c_int as ::core::ffi::c_double / SQRTPI
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l)
        * CINTcommon_fac_sp((*envs).k_l) * CINTcommon_fac_sp((*envs).l_l);
    if *env.offset(PTR_EXPCUTOFF as isize)
        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        (*envs).expcutoff = EXPCUTOFF as ::core::ffi::c_double;
    } else {
        (*envs).expcutoff = (if 40 as ::core::ffi::c_int as ::core::ffi::c_double
            > *env.offset(0 as ::core::ffi::c_int as isize)
        {
            40 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            *env.offset(0 as ::core::ffi::c_int as isize)
        }) + 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    (*envs).gbits = *ng.offset(GSHIFT as isize);
    (*envs).ncomp_e1 = *ng.offset(POS_E1 as isize);
    (*envs).ncomp_e2 = *ng.offset(POS_E2 as isize);
    (*envs).ncomp_tensor = *ng.offset(TENSOR as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(IINC as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(JINC as isize);
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(KINC as isize);
    (*envs).ll_ceil = (*envs).l_l + *ng.offset(LINC as isize);
    let mut rys_order: ::core::ffi::c_int = ((*envs).li_ceil + (*envs).lj_ceil
        + (*envs).lk_ceil + (*envs).ll_ceil) / 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    let mut nrys_roots: ::core::ffi::c_int = rys_order;
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && rys_order <= 3 as ::core::ffi::c_int
    {
        nrys_roots *= 2 as ::core::ffi::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    '_c2rust_label: {
        if i_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"i_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                83 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if j_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"j_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if k_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"k_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_2: {
        if l_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"l_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if (*envs).i_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->i_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                87 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_4: {
        if (*envs).j_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->j_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_5: {
        if (*envs).k_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->k_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_6: {
        if (*envs).l_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->l_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                90 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_7: {
        if *bas
            .offset((8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,i_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_8: {
        if *bas
            .offset((8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,j_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_9: {
        if *bas
            .offset((8 as ::core::ffi::c_int * k_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,k_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_10: {
        if *bas
            .offset((8 as ::core::ffi::c_int * l_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,l_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_11: {
        if *bas
            .offset((8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,i_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                95 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_12: {
        if *bas
            .offset((8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,j_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_13: {
        if *bas
            .offset((8 as ::core::ffi::c_int * k_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,k_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_14: {
        if *bas
            .offset((8 as ::core::ffi::c_int * l_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,l_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_15: {
        if rys_order < 32 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"rys_order < MXRYSROOTS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g2e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                99 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    let mut dli: ::core::ffi::c_int = 0;
    let mut dlj: ::core::ffi::c_int = 0;
    let mut dlk: ::core::ffi::c_int = 0;
    let mut dll: ::core::ffi::c_int = 0;
    let mut ibase: ::core::ffi::c_int = ((*envs).li_ceil > (*envs).lj_ceil)
        as ::core::ffi::c_int;
    let mut kbase: ::core::ffi::c_int = ((*envs).lk_ceil > (*envs).ll_ceil)
        as ::core::ffi::c_int;
    if kbase != 0 {
        dlk = (*envs).lk_ceil + (*envs).ll_ceil + 1 as ::core::ffi::c_int;
        dll = (*envs).ll_ceil + 1 as ::core::ffi::c_int;
    } else {
        dlk = (*envs).lk_ceil + 1 as ::core::ffi::c_int;
        dll = (*envs).lk_ceil + (*envs).ll_ceil + 1 as ::core::ffi::c_int;
    }
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as ::core::ffi::c_int;
        dlj = (*envs).lj_ceil + 1 as ::core::ffi::c_int;
    } else {
        dli = (*envs).li_ceil + 1 as ::core::ffi::c_int;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as ::core::ffi::c_int;
    }
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = nrys_roots * dli * dlk;
    (*envs).g_stride_j = nrys_roots * dli * dlk * dll;
    (*envs).g_size = nrys_roots * dli * dlk * dll * dlj;
    if kbase != 0 {
        (*envs).g2d_klmax = (*envs).g_stride_k;
        (*envs).rx_in_rklrx = (*envs).rk;
        (*envs).rkrl[0 as ::core::ffi::c_int as usize] = *(*envs)
            .rk
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).c2rust_unnamed_1.rl.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rkrl[1 as ::core::ffi::c_int as usize] = *(*envs)
            .rk
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).c2rust_unnamed_1.rl.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rkrl[2 as ::core::ffi::c_int as usize] = *(*envs)
            .rk
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).c2rust_unnamed_1.rl.offset(2 as ::core::ffi::c_int as isize);
    } else {
        (*envs).g2d_klmax = (*envs).g_stride_l;
        (*envs).rx_in_rklrx = (*envs).c2rust_unnamed_1.rl;
        (*envs).rkrl[0 as ::core::ffi::c_int as usize] = *(*envs)
            .c2rust_unnamed_1
            .rl
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).rk.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rkrl[1 as ::core::ffi::c_int as usize] = *(*envs)
            .c2rust_unnamed_1
            .rl
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).rk.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rkrl[2 as ::core::ffi::c_int as usize] = *(*envs)
            .c2rust_unnamed_1
            .rl
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).rk.offset(2 as ::core::ffi::c_int as isize);
    }
    if ibase != 0 {
        (*envs).g2d_ijmax = (*envs).g_stride_i;
        (*envs).rx_in_rijrx = (*envs).ri;
        (*envs).rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    } else {
        (*envs).g2d_ijmax = (*envs).g_stride_j;
        (*envs).rx_in_rijrx = (*envs).rj;
        (*envs).rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(2 as ::core::ffi::c_int as isize);
    }
    if rys_order <= 2 as ::core::ffi::c_int {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
        if rys_order != nrys_roots {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if kbase != 0 {
        if ibase != 0 {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_ik2d4d
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        } else {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_kj2d4d
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if ibase != 0 {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_il2d4d
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    } else {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_lj2d4d
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    }
    (*envs).f_g0_2e = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                ::core::ffi::c_double,
                *mut CINTEnvVars,
            ) -> ::core::ffi::c_int,
        >,
        Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    *mut CINTEnvVars,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg2e_index_xyz(
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let i_l: ::core::ffi::c_int = (*envs).i_l;
    let j_l: ::core::ffi::c_int = (*envs).j_l;
    let k_l: ::core::ffi::c_int = (*envs).k_l;
    let l_l: ::core::ffi::c_int = (*envs).l_l;
    let nfi: ::core::ffi::c_int = (*envs).nfi;
    let nfj: ::core::ffi::c_int = (*envs).nfj;
    let nfk: ::core::ffi::c_int = (*envs).c2rust_unnamed.nfk;
    let nfl: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.nfl;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ofx: ::core::ffi::c_int = 0;
    let mut ofkx: ::core::ffi::c_int = 0;
    let mut oflx: ::core::ffi::c_int = 0;
    let mut ofy: ::core::ffi::c_int = 0;
    let mut ofky: ::core::ffi::c_int = 0;
    let mut ofly: ::core::ffi::c_int = 0;
    let mut ofz: ::core::ffi::c_int = 0;
    let mut ofkz: ::core::ffi::c_int = 0;
    let mut oflz: ::core::ffi::c_int = 0;
    let mut i_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut i_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut i_nz: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_nz: [::core::ffi::c_int; 136] = [0; 136];
    let mut k_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut k_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut k_nz: [::core::ffi::c_int; 136] = [0; 136];
    let mut l_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut l_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut l_nz: [::core::ffi::c_int; 136] = [0; 136];
    CINTcart_comp(
        &raw mut i_nx as *mut ::core::ffi::c_int,
        &raw mut i_ny as *mut ::core::ffi::c_int,
        &raw mut i_nz as *mut ::core::ffi::c_int,
        i_l,
    );
    CINTcart_comp(
        &raw mut j_nx as *mut ::core::ffi::c_int,
        &raw mut j_ny as *mut ::core::ffi::c_int,
        &raw mut j_nz as *mut ::core::ffi::c_int,
        j_l,
    );
    CINTcart_comp(
        &raw mut k_nx as *mut ::core::ffi::c_int,
        &raw mut k_ny as *mut ::core::ffi::c_int,
        &raw mut k_nz as *mut ::core::ffi::c_int,
        k_l,
    );
    CINTcart_comp(
        &raw mut l_nx as *mut ::core::ffi::c_int,
        &raw mut l_ny as *mut ::core::ffi::c_int,
        &raw mut l_nz as *mut ::core::ffi::c_int,
        l_l,
    );
    ofx = 0 as ::core::ffi::c_int;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < nfj {
        l = 0 as ::core::ffi::c_int;
        while l < nfl {
            oflx = ofx + dj * j_nx[j as usize] + dl * l_nx[l as usize];
            ofly = ofy + dj * j_ny[j as usize] + dl * l_ny[l as usize];
            oflz = ofz + dj * j_nz[j as usize] + dl * l_nz[l as usize];
            k = 0 as ::core::ffi::c_int;
            while k < nfk {
                ofkx = oflx + dk * k_nx[k as usize];
                ofky = ofly + dk * k_ny[k as usize];
                ofkz = oflz + dk * k_nz[k as usize];
                match i_l {
                    0 => {
                        *idx.offset((n + 0 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 1 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as ::core::ffi::c_int) as isize) = ofkz;
                        n += 3 as ::core::ffi::c_int;
                    }
                    1 => {
                        *idx.offset((n + 0 as ::core::ffi::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 1 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as ::core::ffi::c_int) as isize) = ofkz;
                        *idx.offset((n + 3 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 4 as ::core::ffi::c_int) as isize) = ofky + di;
                        *idx.offset((n + 5 as ::core::ffi::c_int) as isize) = ofkz;
                        *idx.offset((n + 6 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 7 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 8 as ::core::ffi::c_int) as isize) = ofkz + di;
                        n += 9 as ::core::ffi::c_int;
                    }
                    2 => {
                        *idx.offset((n + 0 as ::core::ffi::c_int) as isize) = ofkx
                            + di * 2 as ::core::ffi::c_int;
                        *idx.offset((n + 1 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as ::core::ffi::c_int) as isize) = ofkz;
                        *idx.offset((n + 3 as ::core::ffi::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 4 as ::core::ffi::c_int) as isize) = ofky + di;
                        *idx.offset((n + 5 as ::core::ffi::c_int) as isize) = ofkz;
                        *idx.offset((n + 6 as ::core::ffi::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 7 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 8 as ::core::ffi::c_int) as isize) = ofkz + di;
                        *idx.offset((n + 9 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 10 as ::core::ffi::c_int) as isize) = ofky
                            + di * 2 as ::core::ffi::c_int;
                        *idx.offset((n + 11 as ::core::ffi::c_int) as isize) = ofkz;
                        *idx.offset((n + 12 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 13 as ::core::ffi::c_int) as isize) = ofky + di;
                        *idx.offset((n + 14 as ::core::ffi::c_int) as isize) = ofkz + di;
                        *idx.offset((n + 15 as ::core::ffi::c_int) as isize) = ofkx;
                        *idx.offset((n + 16 as ::core::ffi::c_int) as isize) = ofky;
                        *idx.offset((n + 17 as ::core::ffi::c_int) as isize) = ofkz
                            + di * 2 as ::core::ffi::c_int;
                        n += 18 as ::core::ffi::c_int;
                    }
                    _ => {
                        i = 0 as ::core::ffi::c_int;
                        while i < nfi {
                            *idx.offset((n + 0 as ::core::ffi::c_int) as isize) = ofkx
                                + di * i_nx[i as usize];
                            *idx.offset((n + 1 as ::core::ffi::c_int) as isize) = ofky
                                + di * i_ny[i as usize];
                            *idx.offset((n + 2 as ::core::ffi::c_int) as isize) = ofkz
                                + di * i_nz[i as usize];
                            n += 3 as ::core::ffi::c_int;
                            i += 1;
                        }
                    }
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let dm: ::core::ffi::c_int = (*envs).g2d_klmax;
    let dn: ::core::ffi::c_int = (*envs).g2d_ijmax;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut off: ::core::ffi::c_int = 0;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p0x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p0y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p0z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut nb1: ::core::ffi::c_double = 0.;
    let mut mb0: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < nroots {
        *gx.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *gy.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        i += 1;
    }
    let mut s0x: ::core::ffi::c_double = 0.;
    let mut s1x: ::core::ffi::c_double = 0.;
    let mut s2x: ::core::ffi::c_double = 0.;
    let mut t0x: ::core::ffi::c_double = 0.;
    let mut t1x: ::core::ffi::c_double = 0.;
    let mut s0y: ::core::ffi::c_double = 0.;
    let mut s1y: ::core::ffi::c_double = 0.;
    let mut s2y: ::core::ffi::c_double = 0.;
    let mut t0y: ::core::ffi::c_double = 0.;
    let mut t1y: ::core::ffi::c_double = 0.;
    let mut s0z: ::core::ffi::c_double = 0.;
    let mut s1z: ::core::ffi::c_double = 0.;
    let mut s2z: ::core::ffi::c_double = 0.;
    let mut t0z: ::core::ffi::c_double = 0.;
    let mut t1z: ::core::ffi::c_double = 0.;
    let mut c00x: ::core::ffi::c_double = 0.;
    let mut c00y: ::core::ffi::c_double = 0.;
    let mut c00z: ::core::ffi::c_double = 0.;
    let mut c0px: ::core::ffi::c_double = 0.;
    let mut c0py: ::core::ffi::c_double = 0.;
    let mut c0pz: ::core::ffi::c_double = 0.;
    let mut b10: ::core::ffi::c_double = 0.;
    let mut b01: ::core::ffi::c_double = 0.;
    let mut b00: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < nroots {
        c00x = (*bc).c00x[i as usize];
        c00y = (*bc).c00y[i as usize];
        c00z = (*bc).c00z[i as usize];
        c0px = (*bc).c0px[i as usize];
        c0py = (*bc).c0py[i as usize];
        c0pz = (*bc).c0pz[i as usize];
        b10 = (*bc).b10[i as usize];
        b01 = (*bc).b01[i as usize];
        b00 = (*bc).b00[i as usize];
        if nmax > 0 as ::core::ffi::c_int {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c00x * s0x;
            s1y = c00y * s0y;
            s1z = c00z * s0z;
            *gx.offset((i + dn) as isize) = s1x;
            *gy.offset((i + dn) as isize) = s1y;
            *gz.offset((i + dn) as isize) = s1z;
            n = 1 as ::core::ffi::c_int;
            while n < nmax {
                s2x = c00x * s1x + n as ::core::ffi::c_double * b10 * s0x;
                s2y = c00y * s1y + n as ::core::ffi::c_double * b10 * s0y;
                s2z = c00z * s1z + n as ::core::ffi::c_double * b10 * s0z;
                *gx.offset((i + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2x;
                *gy.offset((i + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2y;
                *gz.offset((i + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
            }
        }
        if mmax > 0 as ::core::ffi::c_int {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c0px * s0x;
            s1y = c0py * s0y;
            s1z = c0pz * s0z;
            *gx.offset((i + dm) as isize) = s1x;
            *gy.offset((i + dm) as isize) = s1y;
            *gz.offset((i + dm) as isize) = s1z;
            m = 1 as ::core::ffi::c_int;
            while m < mmax {
                s2x = c0px * s1x + m as ::core::ffi::c_double * b01 * s0x;
                s2y = c0py * s1y + m as ::core::ffi::c_double * b01 * s0y;
                s2z = c0pz * s1z + m as ::core::ffi::c_double * b01 * s0z;
                *gx.offset((i + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2x;
                *gy.offset((i + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2y;
                *gz.offset((i + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                m += 1;
            }
            if nmax > 0 as ::core::ffi::c_int {
                s0x = *gx.offset((i + dn) as isize);
                s0y = *gy.offset((i + dn) as isize);
                s0z = *gz.offset((i + dn) as isize);
                s1x = c0px * s0x + b00 * *gx.offset(i as isize);
                s1y = c0py * s0y + b00 * *gy.offset(i as isize);
                s1z = c0pz * s0z + b00 * *gz.offset(i as isize);
                *gx.offset((i + dn + dm) as isize) = s1x;
                *gy.offset((i + dn + dm) as isize) = s1y;
                *gz.offset((i + dn + dm) as isize) = s1z;
                m = 1 as ::core::ffi::c_int;
                while m < mmax {
                    s2x = c0px * s1x + m as ::core::ffi::c_double * b01 * s0x
                        + b00 * *gx.offset((i + m * dm) as isize);
                    s2y = c0py * s1y + m as ::core::ffi::c_double * b01 * s0y
                        + b00 * *gy.offset((i + m * dm) as isize);
                    s2z = c0pz * s1z + m as ::core::ffi::c_double * b01 * s0z
                        + b00 * *gz.offset((i + m * dm) as isize);
                    *gx.offset((i + dn + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2x;
                    *gy.offset((i + dn + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2y;
                    *gz.offset((i + dn + (m + 1 as ::core::ffi::c_int) * dm) as isize) = s2z;
                    s0x = s1x;
                    s0y = s1y;
                    s0z = s1z;
                    s1x = s2x;
                    s1y = s2y;
                    s1z = s2z;
                    m += 1;
                }
            }
        }
        m = 1 as ::core::ffi::c_int;
        while m <= mmax {
            off = m * dm;
            j = off + i;
            s0x = *gx.offset(j as isize);
            s0y = *gy.offset(j as isize);
            s0z = *gz.offset(j as isize);
            s1x = *gx.offset((j + dn) as isize);
            s1y = *gy.offset((j + dn) as isize);
            s1z = *gz.offset((j + dn) as isize);
            n = 1 as ::core::ffi::c_int;
            while n < nmax {
                s2x = c00x * s1x + n as ::core::ffi::c_double * b10 * s0x
                    + m as ::core::ffi::c_double * b00
                        * *gx.offset((j + n * dn - dm) as isize);
                s2y = c00y * s1y + n as ::core::ffi::c_double * b10 * s0y
                    + m as ::core::ffi::c_double * b00
                        * *gy.offset((j + n * dn - dm) as isize);
                s2z = c00z * s1z + n as ::core::ffi::c_double * b10 * s0z
                    + m as ::core::ffi::c_double * b00
                        * *gz.offset((j + n * dn - dm) as isize);
                *gx.offset((j + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2x;
                *gy.offset((j + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2y;
                *gz.offset((j + (n + 1 as ::core::ffi::c_int) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
            }
            m += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_lj2d_4d(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: ::core::ffi::c_int = (*envs).li_ceil;
    let mut lk: ::core::ffi::c_int = (*envs).lk_ceil;
    if li == 0 as ::core::ffi::c_int && lk == 0 as ::core::ffi::c_int {
        return;
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: ::core::ffi::c_int = (*envs).lj_ceil;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = (*envs).g_stride_i;
    let mut dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let mut dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
    let mut rkrl: *mut ::core::ffi::c_double = &raw mut (*envs).rkrl
        as *mut ::core::ffi::c_double;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rx: ::core::ffi::c_double = 0.;
    let mut ry: ::core::ffi::c_double = 0.;
    let mut rz: ::core::ffi::c_double = 0.;
    rx = *rirj.offset(0 as ::core::ffi::c_int as isize);
    ry = *rirj.offset(1 as ::core::ffi::c_int as isize);
    rz = *rirj.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as ::core::ffi::c_int;
    while i <= li {
        j = 0 as ::core::ffi::c_int;
        while j <= nmax - i {
            l = 0 as ::core::ffi::c_int;
            while l <= mmax {
                ptr = j * dj + l * dl + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                l += 1;
            }
            j += 1;
        }
        i += 1;
    }
    rx = *rkrl.offset(0 as ::core::ffi::c_int as isize);
    ry = *rkrl.offset(1 as ::core::ffi::c_int as isize);
    rz = *rkrl.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        k = 1 as ::core::ffi::c_int;
        while k <= lk {
            l = 0 as ::core::ffi::c_int;
            while l <= mmax - k {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                l += 1;
            }
            k += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_kj2d_4d(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: ::core::ffi::c_int = (*envs).li_ceil;
    let mut ll: ::core::ffi::c_int = (*envs).ll_ceil;
    if li == 0 as ::core::ffi::c_int && ll == 0 as ::core::ffi::c_int {
        return;
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: ::core::ffi::c_int = (*envs).lj_ceil;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = (*envs).g_stride_i;
    let mut dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let mut dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
    let mut rkrl: *mut ::core::ffi::c_double = &raw mut (*envs).rkrl
        as *mut ::core::ffi::c_double;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rx: ::core::ffi::c_double = 0.;
    let mut ry: ::core::ffi::c_double = 0.;
    let mut rz: ::core::ffi::c_double = 0.;
    rx = *rirj.offset(0 as ::core::ffi::c_int as isize);
    ry = *rirj.offset(1 as ::core::ffi::c_int as isize);
    rz = *rirj.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as ::core::ffi::c_int;
    while i <= li {
        j = 0 as ::core::ffi::c_int;
        while j <= nmax - i {
            k = 0 as ::core::ffi::c_int;
            while k <= mmax {
                ptr = j * dj + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    rx = *rkrl.offset(0 as ::core::ffi::c_int as isize);
    ry = *rkrl.offset(1 as ::core::ffi::c_int as isize);
    rz = *rkrl.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 1 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= mmax - l {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_il2d_4d(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut lk: ::core::ffi::c_int = (*envs).lk_ceil;
    let mut lj: ::core::ffi::c_int = (*envs).lj_ceil;
    if lj == 0 as ::core::ffi::c_int && lk == 0 as ::core::ffi::c_int {
        return;
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut ll: ::core::ffi::c_int = (*envs).ll_ceil;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = (*envs).g_stride_i;
    let mut dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let mut dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
    let mut rkrl: *mut ::core::ffi::c_double = &raw mut (*envs).rkrl
        as *mut ::core::ffi::c_double;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rx: ::core::ffi::c_double = 0.;
    let mut ry: ::core::ffi::c_double = 0.;
    let mut rz: ::core::ffi::c_double = 0.;
    rx = *rkrl.offset(0 as ::core::ffi::c_int as isize);
    ry = *rkrl.offset(1 as ::core::ffi::c_int as isize);
    rz = *rkrl.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    k = 1 as ::core::ffi::c_int;
    while k <= lk {
        l = 0 as ::core::ffi::c_int;
        while l <= mmax - k {
            i = 0 as ::core::ffi::c_int;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                i += 1;
            }
            l += 1;
        }
        k += 1;
    }
    rx = *rirj.offset(0 as ::core::ffi::c_int as isize);
    ry = *rirj.offset(1 as ::core::ffi::c_int as isize);
    rz = *rirj.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_ik2d_4d(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut lj: ::core::ffi::c_int = (*envs).lj_ceil;
    let mut ll: ::core::ffi::c_int = (*envs).ll_ceil;
    if lj == 0 as ::core::ffi::c_int && ll == 0 as ::core::ffi::c_int {
        return;
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lk: ::core::ffi::c_int = (*envs).lk_ceil;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = (*envs).g_stride_i;
    let mut dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let mut dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
    let mut rkrl: *mut ::core::ffi::c_double = &raw mut (*envs).rkrl
        as *mut ::core::ffi::c_double;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rx: ::core::ffi::c_double = 0.;
    let mut ry: ::core::ffi::c_double = 0.;
    let mut rz: ::core::ffi::c_double = 0.;
    rx = *rkrl.offset(0 as ::core::ffi::c_int as isize);
    ry = *rkrl.offset(1 as ::core::ffi::c_int as isize);
    rz = *rkrl.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    l = 1 as ::core::ffi::c_int;
    while l <= ll {
        k = 0 as ::core::ffi::c_int;
        while k <= mmax - l {
            i = 0 as ::core::ffi::c_int;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                i += 1;
            }
            k += 1;
        }
        l += 1;
    }
    rx = *rirj.offset(0 as ::core::ffi::c_int as isize);
    ry = *rirj.offset(1 as ::core::ffi::c_int as isize);
    rz = *rirj.offset(2 as ::core::ffi::c_int as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize)
                        + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize)
                        + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize)
                        + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0002(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(7 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(13 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(14 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(15 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(13 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0003(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(4 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(5 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(18 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(19 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0011(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *g
        .offset(28 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *g
        .offset(29 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0012(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *g
        .offset(8 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *g
        .offset(9 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *g
        .offset(40 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *g
        .offset(41 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *g
        .offset(32 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(35 as ::core::ffi::c_int as isize) = *g
        .offset(33 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0020(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(7 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(13 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(14 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(15 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(13 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0021(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *g
        .offset(4 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *g
        .offset(5 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *g
        .offset(20 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *g
        .offset(21 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *g
        .offset(32 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(41 as ::core::ffi::c_int as isize) = *g
        .offset(33 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(42 as ::core::ffi::c_int as isize) = *g
        .offset(34 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *g
        .offset(35 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0030(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(4 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(5 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(18 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(19 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0101(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0102(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(20 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(21 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0110(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0111(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *g
        .offset(16 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *g
        .offset(17 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *g
        .offset(40 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *g
        .offset(41 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(52 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(53 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *g
        .offset(64 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *g
        .offset(65 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(51 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(60 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(61 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0120(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(20 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(21 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0200(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(7 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(13 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(14 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(15 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(13 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0201(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(6 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(7 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(28 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(29 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0210(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(6 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(7 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(28 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(29 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0300(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(4 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(5 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(18 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(19 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1002(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(6 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(7 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(28 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(29 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1011(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *g
        .offset(10 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *g
        .offset(11 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(34 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(35 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(51 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *g
        .offset(56 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *g
        .offset(57 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(58 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(59 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(53 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(50 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(51 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1020(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(6 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(7 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(28 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(29 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *g
        .offset(28 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *g
        .offset(29 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1101(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *g
        .offset(12 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *g
        .offset(13 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *g
        .offset(56 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *g
        .offset(57 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(51 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(60 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(61 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *cpz.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *cpz.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1110(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *g
        .offset(12 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *g
        .offset(13 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *g
        .offset(56 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *g
        .offset(57 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(51 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(60 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(61 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *cpz.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *cpz.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1200(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *g
        .offset(8 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *g
        .offset(9 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(2 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *g
        .offset(40 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *g
        .offset(41 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *g
        .offset(32 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(35 as ::core::ffi::c_int as isize) = *g
        .offset(33 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(7 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(13 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(14 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(12 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(15 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(13 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(20 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(21 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(20 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(21 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *g
        .offset(4 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *g
        .offset(5 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *g
        .offset(20 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *g
        .offset(21 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *g
        .offset(36 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *g
        .offset(37 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *g
        .offset(34 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *g
        .offset(35 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *g
        .offset(32 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(41 as ::core::ffi::c_int as isize) = *g
        .offset(33 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_3000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(4 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(5 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(8 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(9 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(18 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(16 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(19 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(17 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(20 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(18 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(21 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(19 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d4d_unrolled(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: ::core::ffi::c_int = (*envs).li_ceil << 6 as ::core::ffi::c_int
        | (*envs).lj_ceil << 4 as ::core::ffi::c_int
        | (*envs).lk_ceil << 2 as ::core::ffi::c_int | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _g0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _g0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _g0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _g0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _g0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _g0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _g0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _g0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _g0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _g0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _g0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _g0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _g0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _g0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _g0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _g0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _g0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _g0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _g0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _g0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _g0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _g0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _g0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _g0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _g0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _g0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _g0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _g0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _g0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _g0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _g0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _g0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _g0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _g0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _g0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    fprintf(
        stderr,
        b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
            as *const ::core::ffi::c_char,
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil,
    );
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(5 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(8 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(9 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0002(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(15 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0003(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(10 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(11 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(24 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(25 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(26 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(27 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(38 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(39 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(5 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(8 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(9 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0011(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *g
        .offset(56 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *g
        .offset(57 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(58 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(59 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(53 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(50 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(51 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0012(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *g
        .offset(16 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *g
        .offset(17 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *g
        .offset(18 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *cpx.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *g
        .offset(19 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *cpx.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(33 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(34 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(35 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(49 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(51 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(50 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *cpy.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(51 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *cpy.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(80 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(72 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(81 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(73 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(82 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(74 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(83 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(75 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(84 as ::core::ffi::c_int as isize) = *g
        .offset(80 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(72 as ::core::ffi::c_int as isize);
    *g.offset(85 as ::core::ffi::c_int as isize) = *g
        .offset(81 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(73 as ::core::ffi::c_int as isize);
    *g.offset(86 as ::core::ffi::c_int as isize) = *g
        .offset(82 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(74 as ::core::ffi::c_int as isize);
    *g.offset(87 as ::core::ffi::c_int as isize) = *g
        .offset(83 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(75 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *g
        .offset(64 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(69 as ::core::ffi::c_int as isize) = *g
        .offset(65 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(70 as ::core::ffi::c_int as isize) = *g
        .offset(66 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(71 as ::core::ffi::c_int as isize) = *g
        .offset(67 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0020(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(15 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0021(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *g
        .offset(8 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *g
        .offset(9 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(10 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *cpx.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(11 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *cpx.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(33 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(34 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(35 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(49 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(51 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *g
        .offset(40 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *g
        .offset(41 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *g
        .offset(42 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *cpy.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *g
        .offset(43 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *cpy.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(68 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(69 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(70 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(71 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(80 as ::core::ffi::c_int as isize) = *g
        .offset(64 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(81 as ::core::ffi::c_int as isize) = *g
        .offset(65 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(82 as ::core::ffi::c_int as isize) = *g
        .offset(66 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(83 as ::core::ffi::c_int as isize) = *g
        .offset(67 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(84 as ::core::ffi::c_int as isize) = *g
        .offset(68 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(85 as ::core::ffi::c_int as isize) = *g
        .offset(69 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(86 as ::core::ffi::c_int as isize) = *g
        .offset(70 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(87 as ::core::ffi::c_int as isize) = *g
        .offset(71 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(88 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(68 as ::core::ffi::c_int as isize);
    *g.offset(89 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(69 as ::core::ffi::c_int as isize);
    *g.offset(90 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(70 as ::core::ffi::c_int as isize);
    *g.offset(91 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(71 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0030(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(10 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(11 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(24 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(25 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(26 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(27 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b01.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(38 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(39 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(5 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(8 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(9 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0101(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0102(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(16 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(17 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(40 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(41 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(42 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(43 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0110(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0111(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *g
        .offset(32 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *g
        .offset(33 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *g
        .offset(34 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *cpx.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *g
        .offset(35 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *cpx.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(49 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(50 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(51 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(72 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(80 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(81 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(82 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(83 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(84 as ::core::ffi::c_int as isize) = *g
        .offset(80 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(85 as ::core::ffi::c_int as isize) = *g
        .offset(81 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(86 as ::core::ffi::c_int as isize) = *g
        .offset(82 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *cpy.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(87 as ::core::ffi::c_int as isize) = *g
        .offset(83 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *cpy.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(120 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(121 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(122 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(123 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(104 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(105 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(106 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(107 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(128 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(120 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(129 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(121 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(130 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(122 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(131 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(123 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(108 as ::core::ffi::c_int as isize) = *g
        .offset(104 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(109 as ::core::ffi::c_int as isize) = *g
        .offset(105 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(110 as ::core::ffi::c_int as isize) = *g
        .offset(106 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(111 as ::core::ffi::c_int as isize) = *g
        .offset(107 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(132 as ::core::ffi::c_int as isize) = *g
        .offset(128 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(120 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(104 as ::core::ffi::c_int as isize);
    *g.offset(133 as ::core::ffi::c_int as isize) = *g
        .offset(129 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(121 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(105 as ::core::ffi::c_int as isize);
    *g.offset(134 as ::core::ffi::c_int as isize) = *g
        .offset(130 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(122 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(106 as ::core::ffi::c_int as isize);
    *g.offset(135 as ::core::ffi::c_int as isize) = *g
        .offset(131 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(123 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(107 as ::core::ffi::c_int as isize);
    *g.offset(100 as ::core::ffi::c_int as isize) = *g
        .offset(96 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(101 as ::core::ffi::c_int as isize) = *g
        .offset(97 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(102 as ::core::ffi::c_int as isize) = *g
        .offset(98 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(103 as ::core::ffi::c_int as isize) = *g
        .offset(99 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(124 as ::core::ffi::c_int as isize) = *g
        .offset(120 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(125 as ::core::ffi::c_int as isize) = *g
        .offset(121 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(126 as ::core::ffi::c_int as isize) = *g
        .offset(122 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(127 as ::core::ffi::c_int as isize) = *g
        .offset(123 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0120(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(16 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(17 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(40 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(41 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(42 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(43 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0200(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(15 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0201(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(14 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(15 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(36 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(37 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(38 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(39 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(58 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(59 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0210(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(14 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(15 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(36 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(37 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(38 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(39 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(58 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(59 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0300(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(10 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(11 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(24 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(25 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(26 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(27 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(38 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(39 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(3 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(5 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(8 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(9 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1002(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(14 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(15 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(36 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(37 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(38 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(39 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(58 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(59 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1011(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    let mut xkxl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize];
    let mut ykyl: ::core::ffi::c_double = (*envs).rkrl[1 as ::core::ffi::c_int as usize];
    let mut zkzl: ::core::ffi::c_double = (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *g
        .offset(20 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *cpx.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *g
        .offset(21 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *cpx.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *g
        .offset(22 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *cpx.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *g
        .offset(23 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *cpx.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = xkxl
        + *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xkxl + *cpx.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(49 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(50 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(51 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *g
        .offset(68 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *cpy.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *g
        .offset(69 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *cpy.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *g
        .offset(70 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *cpy.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *g
        .offset(71 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *cpy.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = ykyl
        + *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (ykyl + *cpy.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(100 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(101 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(102 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(103 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(112 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(113 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(114 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(115 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(116 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(100 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(117 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(101 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(118 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(102 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(119 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(103 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(120 as ::core::ffi::c_int as isize) = *g
        .offset(112 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(121 as ::core::ffi::c_int as isize) = *g
        .offset(113 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(122 as ::core::ffi::c_int as isize) = *g
        .offset(114 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(123 as ::core::ffi::c_int as isize) = *g
        .offset(115 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(124 as ::core::ffi::c_int as isize) = *g
        .offset(116 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(100 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(112 as ::core::ffi::c_int as isize);
    *g.offset(125 as ::core::ffi::c_int as isize) = *g
        .offset(117 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(101 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(113 as ::core::ffi::c_int as isize);
    *g.offset(126 as ::core::ffi::c_int as isize) = *g
        .offset(118 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(102 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(114 as ::core::ffi::c_int as isize);
    *g.offset(127 as ::core::ffi::c_int as isize) = *g
        .offset(119 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(103 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(115 as ::core::ffi::c_int as isize);
    *g.offset(104 as ::core::ffi::c_int as isize) = *g
        .offset(96 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(105 as ::core::ffi::c_int as isize) = *g
        .offset(97 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(106 as ::core::ffi::c_int as isize) = *g
        .offset(98 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(107 as ::core::ffi::c_int as isize) = *g
        .offset(99 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(108 as ::core::ffi::c_int as isize) = *g
        .offset(100 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(109 as ::core::ffi::c_int as isize) = *g
        .offset(101 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(110 as ::core::ffi::c_int as isize) = *g
        .offset(102 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(111 as ::core::ffi::c_int as isize) = *g
        .offset(103 as ::core::ffi::c_int as isize)
        * (zkzl + *cpz.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1020(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut (*bc).b01
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpx.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpx.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpx.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpx.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(12 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(13 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(14 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(15 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *cpy.offset(0 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *cpy.offset(1 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *cpy.offset(2 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *cpy.offset(3 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(36 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(37 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(38 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(39 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(56 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(57 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(58 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(59 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b01.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(56 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b01.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(57 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b01.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(58 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b01.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(59 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *g
        .offset(56 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *g
        .offset(57 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *g
        .offset(58 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *g
        .offset(59 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(53 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(50 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(51 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1101(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *g
        .offset(26 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *c0x.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *g
        .offset(27 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *c0x.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(49 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(50 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(51 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(64 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *c0y.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *c0y.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(112 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(113 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(114 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(115 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(104 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(105 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(106 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(107 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(120 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(112 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(121 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(113 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(122 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(114 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(123 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(115 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(116 as ::core::ffi::c_int as isize) = *g
        .offset(112 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(117 as ::core::ffi::c_int as isize) = *g
        .offset(113 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(118 as ::core::ffi::c_int as isize) = *g
        .offset(114 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(119 as ::core::ffi::c_int as isize) = *g
        .offset(115 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(100 as ::core::ffi::c_int as isize) = *g
        .offset(96 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(101 as ::core::ffi::c_int as isize) = *g
        .offset(97 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(102 as ::core::ffi::c_int as isize) = *g
        .offset(98 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(103 as ::core::ffi::c_int as isize) = *g
        .offset(99 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(124 as ::core::ffi::c_int as isize) = *g
        .offset(120 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(104 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(112 as ::core::ffi::c_int as isize);
    *g.offset(125 as ::core::ffi::c_int as isize) = *g
        .offset(121 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(105 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(113 as ::core::ffi::c_int as isize);
    *g.offset(126 as ::core::ffi::c_int as isize) = *g
        .offset(122 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(106 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(114 as ::core::ffi::c_int as isize);
    *g.offset(127 as ::core::ffi::c_int as isize) = *g
        .offset(123 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(107 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(115 as ::core::ffi::c_int as isize);
    *g.offset(108 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(104 as ::core::ffi::c_int as isize)
        + *cpz.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(112 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(109 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(105 as ::core::ffi::c_int as isize)
        + *cpz.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(113 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(110 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(106 as ::core::ffi::c_int as isize)
        + *cpz.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(114 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(111 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(107 as ::core::ffi::c_int as isize)
        + *cpz.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(115 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1110(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *g
        .offset(24 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *g
        .offset(25 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *g
        .offset(26 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *c0x.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *g
        .offset(27 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *c0x.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(49 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(50 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(51 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(64 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * *b00.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * *b00.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *c0y.offset(2 as ::core::ffi::c_int as isize)
            * *b00.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *c0y.offset(3 as ::core::ffi::c_int as isize)
            * *b00.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(112 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(113 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(114 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(115 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(104 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(105 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(106 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(107 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(120 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(112 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(121 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(113 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(122 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(114 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(123 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(115 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(116 as ::core::ffi::c_int as isize) = *g
        .offset(112 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(117 as ::core::ffi::c_int as isize) = *g
        .offset(113 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(118 as ::core::ffi::c_int as isize) = *g
        .offset(114 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(119 as ::core::ffi::c_int as isize) = *g
        .offset(115 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
    *g.offset(100 as ::core::ffi::c_int as isize) = *g
        .offset(96 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(101 as ::core::ffi::c_int as isize) = *g
        .offset(97 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(102 as ::core::ffi::c_int as isize) = *g
        .offset(98 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(103 as ::core::ffi::c_int as isize) = *g
        .offset(99 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(124 as ::core::ffi::c_int as isize) = *g
        .offset(120 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(104 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(112 as ::core::ffi::c_int as isize);
    *g.offset(125 as ::core::ffi::c_int as isize) = *g
        .offset(121 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(105 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(113 as ::core::ffi::c_int as isize);
    *g.offset(126 as ::core::ffi::c_int as isize) = *g
        .offset(122 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(106 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(114 as ::core::ffi::c_int as isize);
    *g.offset(127 as ::core::ffi::c_int as isize) = *g
        .offset(123 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(107 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(115 as ::core::ffi::c_int as isize);
    *g.offset(108 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(104 as ::core::ffi::c_int as isize)
        + *cpz.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(112 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(96 as ::core::ffi::c_int as isize);
    *g.offset(109 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(105 as ::core::ffi::c_int as isize)
        + *cpz.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(113 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(97 as ::core::ffi::c_int as isize);
    *g.offset(110 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(106 as ::core::ffi::c_int as isize)
        + *cpz.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(114 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(98 as ::core::ffi::c_int as isize);
    *g.offset(111 as ::core::ffi::c_int as isize) = zizj
        * *g.offset(107 as ::core::ffi::c_int as isize)
        + *cpz.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(115 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(99 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1200(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *g
        .offset(16 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *g
        .offset(17 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *g
        .offset(18 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *c0x.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *g
        .offset(19 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *c0x.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(4 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(33 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(34 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(35 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(49 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(51 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *g
        .offset(48 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *g
        .offset(49 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *g
        .offset(50 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *c0y.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *g
        .offset(51 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *c0y.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(80 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(72 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(81 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(73 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(82 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(74 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(83 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(75 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(84 as ::core::ffi::c_int as isize) = *g
        .offset(80 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(72 as ::core::ffi::c_int as isize);
    *g.offset(85 as ::core::ffi::c_int as isize) = *g
        .offset(81 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(73 as ::core::ffi::c_int as isize);
    *g.offset(86 as ::core::ffi::c_int as isize) = *g
        .offset(82 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(74 as ::core::ffi::c_int as isize);
    *g.offset(87 as ::core::ffi::c_int as isize) = *g
        .offset(83 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(75 as ::core::ffi::c_int as isize);
    *g.offset(76 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(77 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(78 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(79 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *g
        .offset(64 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(69 as ::core::ffi::c_int as isize) = *g
        .offset(65 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(70 as ::core::ffi::c_int as isize) = *g
        .offset(66 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(71 as ::core::ffi::c_int as isize) = *g
        .offset(67 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(13 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(14 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(15 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(16 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(27 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(28 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(24 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(29 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(25 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(30 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(26 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(31 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(27 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2001(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(16 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(17 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(40 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(41 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(42 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(43 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2010(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut cpx: *mut ::core::ffi::c_double = &raw mut (*bc).c0px
        as *mut ::core::ffi::c_double;
    let mut cpy: *mut ::core::ffi::c_double = &raw mut (*bc).c0py
        as *mut ::core::ffi::c_double;
    let mut cpz: *mut ::core::ffi::c_double = &raw mut (*bc).c0pz
        as *mut ::core::ffi::c_double;
    let mut b00: *mut ::core::ffi::c_double = &raw mut (*bc).b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(13 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(14 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(15 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = *cpx
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = *cpx
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = *cpx
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = *cpx
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(16 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpx.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(17 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpx.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(18 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpx.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(19 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpx.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(25 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(26 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(27 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(33 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(34 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(35 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(36 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *cpy
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *cpy
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *cpy
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *cpy
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(40 as ::core::ffi::c_int as isize)
            + *b00.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *cpy.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(41 as ::core::ffi::c_int as isize)
            + *b00.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *cpy.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(42 as ::core::ffi::c_int as isize)
            + *b00.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *cpy.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(43 as ::core::ffi::c_int as isize)
            + *b00.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *cpy.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(60 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(61 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(62 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(63 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(64 as ::core::ffi::c_int as isize) = *cpz
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(52 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(48 as ::core::ffi::c_int as isize);
    *g.offset(65 as ::core::ffi::c_int as isize) = *cpz
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(53 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(49 as ::core::ffi::c_int as isize);
    *g.offset(66 as ::core::ffi::c_int as isize) = *cpz
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(54 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(50 as ::core::ffi::c_int as isize);
    *g.offset(67 as ::core::ffi::c_int as isize) = *cpz
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(55 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(51 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(60 as ::core::ffi::c_int as isize)
        + *b00.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(52 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(61 as ::core::ffi::c_int as isize)
        + *b00.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(53 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(62 as ::core::ffi::c_int as isize)
        + *b00.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(54 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(63 as ::core::ffi::c_int as isize)
        + *b00.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(55 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2100(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    let mut xixj: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize];
    let mut yiyj: ::core::ffi::c_double = (*envs).rirj[1 as ::core::ffi::c_int as usize];
    let mut zizj: ::core::ffi::c_double = (*envs).rirj[2 as ::core::ffi::c_int as usize];
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *g
        .offset(8 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *c0x.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *g
        .offset(9 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *c0x.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *g
        .offset(10 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *c0x.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *g
        .offset(11 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *c0x.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (xixj + *c0x.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(16 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(17 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(18 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(19 as ::core::ffi::c_int as isize) = xixj
        + *c0x.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(32 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(33 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(34 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(35 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(56 as ::core::ffi::c_int as isize) = *g
        .offset(40 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *c0y.offset(0 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(57 as ::core::ffi::c_int as isize) = *g
        .offset(41 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *c0y.offset(1 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(58 as ::core::ffi::c_int as isize) = *g
        .offset(42 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *c0y.offset(2 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(59 as ::core::ffi::c_int as isize) = *g
        .offset(43 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *c0y.offset(3 as ::core::ffi::c_int as isize)
            * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(52 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(53 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(54 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(55 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (yiyj + *c0y.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(48 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(49 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(50 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(51 as ::core::ffi::c_int as isize) = yiyj
        + *c0y.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(68 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(69 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(70 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(71 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(72 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(68 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(73 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(69 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(74 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(70 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(75 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(71 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(88 as ::core::ffi::c_int as isize) = *g
        .offset(72 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(68 as ::core::ffi::c_int as isize);
    *g.offset(89 as ::core::ffi::c_int as isize) = *g
        .offset(73 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(69 as ::core::ffi::c_int as isize);
    *g.offset(90 as ::core::ffi::c_int as isize) = *g
        .offset(74 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(70 as ::core::ffi::c_int as isize);
    *g.offset(91 as ::core::ffi::c_int as isize) = *g
        .offset(75 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(71 as ::core::ffi::c_int as isize);
    *g.offset(84 as ::core::ffi::c_int as isize) = *g
        .offset(68 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize))
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(64 as ::core::ffi::c_int as isize);
    *g.offset(85 as ::core::ffi::c_int as isize) = *g
        .offset(69 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize))
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(65 as ::core::ffi::c_int as isize);
    *g.offset(86 as ::core::ffi::c_int as isize) = *g
        .offset(70 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize))
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(66 as ::core::ffi::c_int as isize);
    *g.offset(87 as ::core::ffi::c_int as isize) = *g
        .offset(71 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize))
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(67 as ::core::ffi::c_int as isize);
    *g.offset(80 as ::core::ffi::c_int as isize) = *g
        .offset(64 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(81 as ::core::ffi::c_int as isize) = *g
        .offset(65 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(82 as ::core::ffi::c_int as isize) = *g
        .offset(66 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(83 as ::core::ffi::c_int as isize) = *g
        .offset(67 as ::core::ffi::c_int as isize)
        * (zizj + *c0z.offset(3 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_3000(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut ::core::ffi::c_double = &raw mut (*bc).c00x
        as *mut ::core::ffi::c_double;
    let mut c0y: *mut ::core::ffi::c_double = &raw mut (*bc).c00y
        as *mut ::core::ffi::c_double;
    let mut c0z: *mut ::core::ffi::c_double = &raw mut (*bc).c00z
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut (*bc).b10
        as *mut ::core::ffi::c_double;
    *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(4 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(5 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(6 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(7 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(8 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0x.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(9 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0x.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(10 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0x.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(11 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0x.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(12 as ::core::ffi::c_int as isize) = *c0x
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(8 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(13 as ::core::ffi::c_int as isize) = *c0x
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(9 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(14 as ::core::ffi::c_int as isize) = *c0x
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(10 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(15 as ::core::ffi::c_int as isize) = *c0x
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(11 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(16 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(17 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *g.offset(20 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize);
    *g.offset(21 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize);
    *g.offset(22 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize);
    *g.offset(23 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize);
    *g.offset(24 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * *c0y.offset(0 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize);
    *g.offset(25 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * *c0y.offset(1 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize);
    *g.offset(26 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * *c0y.offset(2 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize);
    *g.offset(27 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * *c0y.offset(3 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize);
    *g.offset(28 as ::core::ffi::c_int as isize) = *c0y
        .offset(0 as ::core::ffi::c_int as isize)
        * (*g.offset(24 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(0 as ::core::ffi::c_int as isize));
    *g.offset(29 as ::core::ffi::c_int as isize) = *c0y
        .offset(1 as ::core::ffi::c_int as isize)
        * (*g.offset(25 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(1 as ::core::ffi::c_int as isize));
    *g.offset(30 as ::core::ffi::c_int as isize) = *c0y
        .offset(2 as ::core::ffi::c_int as isize)
        * (*g.offset(26 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(2 as ::core::ffi::c_int as isize));
    *g.offset(31 as ::core::ffi::c_int as isize) = *c0y
        .offset(3 as ::core::ffi::c_int as isize)
        * (*g.offset(27 as ::core::ffi::c_int as isize)
            + 2 as ::core::ffi::c_int as ::core::ffi::c_double
                * *b10.offset(3 as ::core::ffi::c_int as isize));
    *g.offset(36 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(37 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(38 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(39 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(40 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(36 as ::core::ffi::c_int as isize)
        + *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(32 as ::core::ffi::c_int as isize);
    *g.offset(41 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(37 as ::core::ffi::c_int as isize)
        + *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(33 as ::core::ffi::c_int as isize);
    *g.offset(42 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(38 as ::core::ffi::c_int as isize)
        + *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(34 as ::core::ffi::c_int as isize);
    *g.offset(43 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(39 as ::core::ffi::c_int as isize)
        + *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(35 as ::core::ffi::c_int as isize);
    *g.offset(44 as ::core::ffi::c_int as isize) = *c0z
        .offset(0 as ::core::ffi::c_int as isize)
        * *g.offset(40 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(0 as ::core::ffi::c_int as isize)
            * *g.offset(36 as ::core::ffi::c_int as isize);
    *g.offset(45 as ::core::ffi::c_int as isize) = *c0z
        .offset(1 as ::core::ffi::c_int as isize)
        * *g.offset(41 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(1 as ::core::ffi::c_int as isize)
            * *g.offset(37 as ::core::ffi::c_int as isize);
    *g.offset(46 as ::core::ffi::c_int as isize) = *c0z
        .offset(2 as ::core::ffi::c_int as isize)
        * *g.offset(42 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(2 as ::core::ffi::c_int as isize)
            * *g.offset(38 as ::core::ffi::c_int as isize);
    *g.offset(47 as ::core::ffi::c_int as isize) = *c0z
        .offset(3 as ::core::ffi::c_int as isize)
        * *g.offset(43 as ::core::ffi::c_int as isize)
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double
            * *b10.offset(3 as ::core::ffi::c_int as isize)
            * *g.offset(39 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTsrg0_2e_2d4d_unrolled(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: ::core::ffi::c_int = (*envs).li_ceil << 6 as ::core::ffi::c_int
        | (*envs).lj_ceil << 4 as ::core::ffi::c_int
        | (*envs).lk_ceil << 2 as ::core::ffi::c_int | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _srg0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _srg0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _srg0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _srg0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _srg0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _srg0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _srg0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _srg0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _srg0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _srg0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _srg0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _srg0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _srg0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _srg0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _srg0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _srg0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _srg0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _srg0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _srg0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _srg0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _srg0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _srg0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _srg0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _srg0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _srg0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _srg0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _srg0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _srg0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _srg0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _srg0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _srg0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _srg0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _srg0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _srg0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _srg0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    fprintf(
        stderr,
        b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
            as *const ::core::ffi::c_char,
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_lj2d4d(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_lj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_kj2d4d(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_kj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_ik2d4d(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_ik2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_il2d4d(
    mut g: *mut ::core::ffi::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_il2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e(
    mut g: *mut ::core::ffi::c_double,
    mut rij: *mut ::core::ffi::c_double,
    mut rkl: *mut ::core::ffi::c_double,
    mut cutoff: ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) -> ::core::ffi::c_int {
    let mut irys: ::core::ffi::c_int = 0;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut aij: ::core::ffi::c_double = (*envs).ai[0 as ::core::ffi::c_int as usize]
        + (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut akl: ::core::ffi::c_double = (*envs).ak[0 as ::core::ffi::c_int as usize]
        + (*envs).al[0 as ::core::ffi::c_int as usize];
    let mut a0: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut fac1: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut u: [::core::ffi::c_double; 32] = [0.; 32];
    let mut w: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut xij_kl: ::core::ffi::c_double = *rij.offset(0 as ::core::ffi::c_int as isize)
        - *rkl.offset(0 as ::core::ffi::c_int as isize);
    let mut yij_kl: ::core::ffi::c_double = *rij.offset(1 as ::core::ffi::c_int as isize)
        - *rkl.offset(1 as ::core::ffi::c_int as isize);
    let mut zij_kl: ::core::ffi::c_double = *rij.offset(2 as ::core::ffi::c_int as isize)
        - *rkl.offset(2 as ::core::ffi::c_int as isize);
    let mut rr: ::core::ffi::c_double = xij_kl * xij_kl + yij_kl * yij_kl
        + zij_kl * zij_kl;
    a1 = aij * akl;
    a0 = a1 / (aij + akl);
    fac1 = sqrt(a0 / (a1 * a1 * a1)) * (*envs).fac[0 as ::core::ffi::c_int as usize];
    x = a0 * rr;
    let omega: ::core::ffi::c_double = *(*envs).env.offset(PTR_RANGE_OMEGA as isize);
    let mut theta: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    if omega == 0.0f64 {
        let mut err: ::core::ffi::c_int = CINTrys_roots(
            nroots,
            x,
            &raw mut u as *mut ::core::ffi::c_double,
            w,
        );
        if err != 0 {
            fprintf(
                stderr,
                b"rys_roots failed. Integrals for l=(%d,%d|%d,%d) exp=(%g,%g|%g,%g) are set to 0\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*envs).li_ceil,
                (*envs).lj_ceil,
                (*envs).lk_ceil,
                (*envs).ll_ceil,
                (*envs).ai[0 as ::core::ffi::c_int as usize],
                (*envs).aj[0 as ::core::ffi::c_int as usize],
                (*envs).ak[0 as ::core::ffi::c_int as usize],
                (*envs).al[0 as ::core::ffi::c_int as usize],
            );
            return 0 as ::core::ffi::c_int;
        }
    } else if omega < 0.0f64 {
        theta = omega * omega / (omega * omega + a0);
        if theta * x > cutoff || theta * x > EXPCUTOFF_SR as ::core::ffi::c_double {
            return 0 as ::core::ffi::c_int;
        }
        let mut rorder: ::core::ffi::c_int = (*envs).rys_order;
        if rorder == nroots {
            let mut err_0: ::core::ffi::c_int = CINTsr_rys_roots(
                nroots,
                x,
                sqrt(theta),
                &raw mut u as *mut ::core::ffi::c_double,
                w,
            );
            if err_0 != 0 {
                fprintf(
                    stderr,
                    b"sr_rys_roots failed. Integrals for l=(%d,%d|%d,%d) exp=(%g,%g|%g,%g) are set to 0\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*envs).li_ceil,
                    (*envs).lj_ceil,
                    (*envs).lk_ceil,
                    (*envs).ll_ceil,
                    (*envs).ai[0 as ::core::ffi::c_int as usize],
                    (*envs).aj[0 as ::core::ffi::c_int as usize],
                    (*envs).ak[0 as ::core::ffi::c_int as usize],
                    (*envs).al[0 as ::core::ffi::c_int as usize],
                );
                return 0 as ::core::ffi::c_int;
            }
        } else {
            let mut sqrt_theta: ::core::ffi::c_double = -sqrt(theta);
            let mut err_1: ::core::ffi::c_int = CINTrys_roots(
                rorder,
                x,
                &raw mut u as *mut ::core::ffi::c_double,
                w,
            );
            CINTrys_roots(
                rorder,
                theta * x,
                (&raw mut u as *mut ::core::ffi::c_double).offset(rorder as isize),
                w.offset(rorder as isize),
            );
            if (*envs).g_size == 2 as ::core::ffi::c_int {
                *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                *g.offset(2 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                *g.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                *g.offset(4 as ::core::ffi::c_int as isize) *= fac1;
                *g.offset(5 as ::core::ffi::c_int as isize) *= fac1 * sqrt_theta;
                return 1 as ::core::ffi::c_int;
            }
            irys = rorder;
            while irys < nroots {
                let mut ut: ::core::ffi::c_double = u[irys as usize] * theta;
                u[irys as usize] = ut / (u[irys as usize] + 1.0f64 - ut);
                *w.offset(irys as isize) *= sqrt_theta;
                irys += 1;
            }
        }
    } else {
        theta = omega * omega / (omega * omega + a0);
        x *= theta;
        fac1 *= sqrt(theta);
        let mut err_2: ::core::ffi::c_int = CINTrys_roots(
            nroots,
            x,
            &raw mut u as *mut ::core::ffi::c_double,
            w,
        );
        if err_2 != 0 {
            fprintf(
                stderr,
                b"lr_rys_roots failed. Integrals for l=(%d,%d|%d,%d) exp=(%g,%g|%g,%g) are set to 0\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*envs).li_ceil,
                (*envs).lj_ceil,
                (*envs).lk_ceil,
                (*envs).ll_ceil,
                (*envs).ai[0 as ::core::ffi::c_int as usize],
                (*envs).aj[0 as ::core::ffi::c_int as usize],
                (*envs).ak[0 as ::core::ffi::c_int as usize],
                (*envs).al[0 as ::core::ffi::c_int as usize],
            );
            return 0 as ::core::ffi::c_int;
        }
        irys = 0 as ::core::ffi::c_int;
        while irys < nroots {
            let mut ut_0: ::core::ffi::c_double = u[irys as usize] * theta;
            u[irys as usize] = ut_0 / (u[irys as usize] + 1.0f64 - ut_0);
            irys += 1;
        }
    }
    if (*envs).g_size == 1 as ::core::ffi::c_int {
        *g.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        *g.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        *g.offset(2 as ::core::ffi::c_int as isize) *= fac1;
        return 1 as ::core::ffi::c_int;
    }
    let mut u2: ::core::ffi::c_double = 0.;
    let mut tmp1: ::core::ffi::c_double = 0.;
    let mut tmp2: ::core::ffi::c_double = 0.;
    let mut tmp3: ::core::ffi::c_double = 0.;
    let mut tmp4: ::core::ffi::c_double = 0.;
    let mut tmp5: ::core::ffi::c_double = 0.;
    let mut rijrx: ::core::ffi::c_double = *rij.offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rijrx.offset(0 as ::core::ffi::c_int as isize);
    let mut rijry: ::core::ffi::c_double = *rij.offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rijrx.offset(1 as ::core::ffi::c_int as isize);
    let mut rijrz: ::core::ffi::c_double = *rij.offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rijrx.offset(2 as ::core::ffi::c_int as isize);
    let mut rklrx: ::core::ffi::c_double = *rkl.offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rklrx.offset(0 as ::core::ffi::c_int as isize);
    let mut rklry: ::core::ffi::c_double = *rkl.offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rklrx.offset(1 as ::core::ffi::c_int as isize);
    let mut rklrz: ::core::ffi::c_double = *rkl.offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rx_in_rklrx.offset(2 as ::core::ffi::c_int as isize);
    let mut bc: Rys2eT = Rys2eT {
        c00x: [0.; 32],
        c00y: [0.; 32],
        c00z: [0.; 32],
        c0px: [0.; 32],
        c0py: [0.; 32],
        c0pz: [0.; 32],
        b01: [0.; 32],
        b00: [0.; 32],
        b10: [0.; 32],
    };
    let mut b00: *mut ::core::ffi::c_double = &raw mut bc.b00
        as *mut ::core::ffi::c_double;
    let mut b10: *mut ::core::ffi::c_double = &raw mut bc.b10
        as *mut ::core::ffi::c_double;
    let mut b01: *mut ::core::ffi::c_double = &raw mut bc.b01
        as *mut ::core::ffi::c_double;
    let mut c00x: *mut ::core::ffi::c_double = &raw mut bc.c00x
        as *mut ::core::ffi::c_double;
    let mut c00y: *mut ::core::ffi::c_double = &raw mut bc.c00y
        as *mut ::core::ffi::c_double;
    let mut c00z: *mut ::core::ffi::c_double = &raw mut bc.c00z
        as *mut ::core::ffi::c_double;
    let mut c0px: *mut ::core::ffi::c_double = &raw mut bc.c0px
        as *mut ::core::ffi::c_double;
    let mut c0py: *mut ::core::ffi::c_double = &raw mut bc.c0py
        as *mut ::core::ffi::c_double;
    let mut c0pz: *mut ::core::ffi::c_double = &raw mut bc.c0pz
        as *mut ::core::ffi::c_double;
    irys = 0 as ::core::ffi::c_int;
    while irys < nroots {
        u2 = a0 * u[irys as usize];
        tmp4 = 0.5f64 / (u2 * (aij + akl) + a1);
        tmp5 = u2 * tmp4;
        tmp1 = 2.0f64 * tmp5;
        tmp2 = tmp1 * akl;
        tmp3 = tmp1 * aij;
        *b00.offset(irys as isize) = tmp5;
        *b10.offset(irys as isize) = tmp5 + tmp4 * akl;
        *b01.offset(irys as isize) = tmp5 + tmp4 * aij;
        *c00x.offset(irys as isize) = rijrx - tmp2 * xij_kl;
        *c00y.offset(irys as isize) = rijry - tmp2 * yij_kl;
        *c00z.offset(irys as isize) = rijrz - tmp2 * zij_kl;
        *c0px.offset(irys as isize) = rklrx + tmp3 * xij_kl;
        *c0py.offset(irys as isize) = rklry + tmp3 * yij_kl;
        *c0pz.offset(irys as isize) = rklrz + tmp3 * zij_kl;
        *w.offset(irys as isize) *= fac1;
        irys += 1;
    }
    ::core::mem::transmute::<
        _,
        fn(_, _, _),
    >(
        Some((*envs).f_g0_2d4d.expect("non-null function pointer"))
            .expect("non-null function pointer"),
    )(g, &raw mut bc, envs);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let ai2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).ai[0 as ::core::ffi::c_int as usize];
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(-(di as isize));
    let mut p1y: *const ::core::ffi::c_double = gy.offset(-(di as isize));
    let mut p1z: *const ::core::ffi::c_double = gz.offset(-(di as isize));
    let mut p2x: *const ::core::ffi::c_double = gx.offset(di as isize);
    let mut p2y: *const ::core::ffi::c_double = gy.offset(di as isize);
    let mut p2z: *const ::core::ffi::c_double = gz.offset(di as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ai2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ai2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ai2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i = 1 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = i as ::core::ffi::c_double
                            * *p1x.offset(n as isize) + ai2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) = i as ::core::ffi::c_double
                            * *p1y.offset(n as isize) + ai2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) = i as ::core::ffi::c_double
                            * *p1z.offset(n as isize) + ai2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let aj2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(-(dj as isize));
    let mut p1y: *const ::core::ffi::c_double = gy.offset(-(dj as isize));
    let mut p1z: *const ::core::ffi::c_double = gz.offset(-(dj as isize));
    let mut p2x: *const ::core::ffi::c_double = gx.offset(dj as isize);
    let mut p2y: *const ::core::ffi::c_double = gy.offset(dj as isize);
    let mut p2z: *const ::core::ffi::c_double = gz.offset(dj as isize);
    l = 0 as ::core::ffi::c_int;
    while l <= ll {
        k = 0 as ::core::ffi::c_int;
        while k <= lk {
            ptr = dl * l + dk * k;
            i = 0 as ::core::ffi::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = aj2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = aj2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = aj2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k += 1;
        }
        l += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = j as ::core::ffi::c_double
                            * *p1x.offset(n as isize) + aj2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) = j as ::core::ffi::c_double
                            * *p1y.offset(n as isize) + aj2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) = j as ::core::ffi::c_double
                            * *p1z.offset(n as isize) + aj2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let ak2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).ak[0 as ::core::ffi::c_int as usize];
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(-(dk as isize));
    let mut p1y: *const ::core::ffi::c_double = gy.offset(-(dk as isize));
    let mut p1z: *const ::core::ffi::c_double = gz.offset(-(dk as isize));
    let mut p2x: *const ::core::ffi::c_double = gx.offset(dk as isize);
    let mut p2y: *const ::core::ffi::c_double = gy.offset(dk as isize);
    let mut p2z: *const ::core::ffi::c_double = gz.offset(dk as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            ptr = dj * j + dl * l;
            i = 0 as ::core::ffi::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ak2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ak2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ak2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k = 1 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = k as ::core::ffi::c_double
                            * *p1x.offset(n as isize) + ak2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) = k as ::core::ffi::c_double
                            * *p1y.offset(n as isize) + ak2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) = k as ::core::ffi::c_double
                            * *p1z.offset(n as isize) + ak2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1l_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let al2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).al[0 as ::core::ffi::c_int as usize];
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(-(dl as isize));
    let mut p1y: *const ::core::ffi::c_double = gy.offset(-(dl as isize));
    let mut p1z: *const ::core::ffi::c_double = gz.offset(-(dl as isize));
    let mut p2x: *const ::core::ffi::c_double = gx.offset(dl as isize);
    let mut p2y: *const ::core::ffi::c_double = gy.offset(dl as isize);
    let mut p2z: *const ::core::ffi::c_double = gz.offset(dl as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        k = 0 as ::core::ffi::c_int;
        while k <= lk {
            ptr = dj * j + dk * k;
            i = 0 as ::core::ffi::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = al2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = al2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = al2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k += 1;
        }
        l = 1 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = l as ::core::ffi::c_double
                            * *p1x.offset(n as isize) + al2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) = l as ::core::ffi::c_double
                            * *p1y.offset(n as isize) + al2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) = l as ::core::ffi::c_double
                            * *p1z.offset(n as isize) + al2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    i += 1;
                    ptr += di;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    mut ri: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(di as isize);
    let mut p1y: *const ::core::ffi::c_double = gy.offset(di as isize);
    let mut p1z: *const ::core::ffi::c_double = gz.offset(di as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *ri.offset(0 as ::core::ffi::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *ri.offset(1 as ::core::ffi::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *ri.offset(2 as ::core::ffi::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    mut rj: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(dj as isize);
    let mut p1y: *const ::core::ffi::c_double = gy.offset(dj as isize);
    let mut p1z: *const ::core::ffi::c_double = gz.offset(dj as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rj.offset(0 as ::core::ffi::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rj.offset(1 as ::core::ffi::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rj.offset(2 as ::core::ffi::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    mut rk: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(dk as isize);
    let mut p1y: *const ::core::ffi::c_double = gy.offset(dk as isize);
    let mut p1z: *const ::core::ffi::c_double = gz.offset(dk as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rk.offset(0 as ::core::ffi::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rk.offset(1 as ::core::ffi::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rk.offset(2 as ::core::ffi::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1l_2e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *const ::core::ffi::c_double,
    mut rl: *const ::core::ffi::c_double,
    li: ::core::ffi::c_int,
    lj: ::core::ffi::c_int,
    lk: ::core::ffi::c_int,
    ll: ::core::ffi::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let dl: ::core::ffi::c_int = (*envs).g_stride_l;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut p1x: *const ::core::ffi::c_double = gx.offset(dl as isize);
    let mut p1y: *const ::core::ffi::c_double = gy.offset(dl as isize);
    let mut p1z: *const ::core::ffi::c_double = gz.offset(dl as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        l = 0 as ::core::ffi::c_int;
        while l <= ll {
            k = 0 as ::core::ffi::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as ::core::ffi::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rl.offset(0 as ::core::ffi::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rl.offset(1 as ::core::ffi::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rl.offset(2 as ::core::ffi::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
pub const SQRTPI: ::core::ffi::c_double = 1.7724538509055160272981674833411451f64;
pub const IINC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JINC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KINC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LINC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const GSHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POS_E1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const POS_E2: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TENSOR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EXPCUTOFF: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
