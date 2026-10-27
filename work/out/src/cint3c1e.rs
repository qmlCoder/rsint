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
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn CINTinit_int3c1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINTprim_to_ctr_0(
        gc: *mut ::core::ffi::c_double,
        gp: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nf: size_t,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
        non0ctr: ::core::ffi::c_int,
        sortedidx: *mut ::core::ffi::c_int,
    );
    fn CINTprim_to_ctr_1(
        gc: *mut ::core::ffi::c_double,
        gp: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nf: size_t,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
        non0ctr: ::core::ffi::c_int,
        sortedidx: *mut ::core::ffi::c_int,
    );
    fn CINTg2e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *const CINTEnvVars);
    fn CINTOpt_non0coeff_byshell(
        sortedidx: *mut ::core::ffi::c_int,
        non0ctr: *mut ::core::ffi::c_int,
        ci: *mut ::core::ffi::c_double,
        iprim: ::core::ffi::c_int,
        ictr: ::core::ffi::c_int,
    );
    fn CINTnuc_mod(
        aij: ::core::ffi::c_double,
        nuc_id: ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn CINTgout1e(
        gout: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        idx: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        empty: ::core::ffi::c_int,
    );
    fn CINTg3c1e_ovlp(
        g: *mut ::core::ffi::c_double,
        ai: ::core::ffi::c_double,
        aj: ::core::ffi::c_double,
        ak: ::core::ffi::c_double,
        envs: *mut CINTEnvVars,
    );
    fn CINTg3c1e_nuc(
        g: *mut ::core::ffi::c_double,
        ai: ::core::ffi::c_double,
        aj: ::core::ffi::c_double,
        ak: ::core::ffi::c_double,
        rijk: *mut ::core::ffi::c_double,
        cr: *mut ::core::ffi::c_double,
        t2: ::core::ffi::c_double,
        envs: *mut CINTEnvVars,
    );
    fn CINTdmat_transpose(
        a_t: *mut ::core::ffi::c_double,
        a: *mut ::core::ffi::c_double,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    );
    fn CINTdplus_transpose(
        a_t: *mut ::core::ffi::c_double,
        a: *mut ::core::ffi::c_double,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    );
    fn CINTsquare_dist(
        r1: *const ::core::ffi::c_double,
        r2: *const ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn c2s_sph_3c2e1(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_sph_3c1e(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_cart_3c1e(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_dset0(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        counts: *mut ::core::ffi::c_int,
    );
    fn CINTrys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        u: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
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
pub type uintptr_t = usize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PTR_RINV_ORIG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ATM_SLOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const INT1E_TYPE_OVLP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT1E_TYPE_RINV: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINT3c1e_loop_nopt(
    mut gctr: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut ::core::ffi::c_double,
    mut empty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut shls: *mut ::core::ffi::c_int = (*envs).shls;
    let mut bas: *mut ::core::ffi::c_int = (*envs).bas;
    let mut env: *mut ::core::ffi::c_double = (*envs).env;
    let mut i_sh: ::core::ffi::c_int = *shls.offset(0 as ::core::ffi::c_int as isize);
    let mut j_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    let mut k_sh: ::core::ffi::c_int = *shls.offset(2 as ::core::ffi::c_int as isize);
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut ri: *mut ::core::ffi::c_double = (*envs).ri;
    let mut rj: *mut ::core::ffi::c_double = (*envs).rj;
    let mut rk: *mut ::core::ffi::c_double = (*envs).rk;
    let mut ai: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut aj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ak: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * k_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ci: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut cj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ck: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * k_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 4] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut iempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize);
    let mut jempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut kempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(2 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = idx.offset(((*envs).nf * 3 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_double;
    CINTg2e_index_xyz(idx, envs);
    let mut non0ctri: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0ctrj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0ctrk: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxi: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxk: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + i_prim * i_ctr + j_prim * j_ctr + k_prim * k_ctr)
                as isize,
        ) as *mut ::core::ffi::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut nc: ::core::ffi::c_int = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
    let mut len: size_t = leng
        .wrapping_add(lenk)
        .wrapping_add(lenj)
        .wrapping_add(leni)
        .wrapping_add(len0);
    let mut g: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    g = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut ::core::ffi::c_double = g.offset(leng as isize);
    let mut gout: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctri: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctrj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctrk: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as ::core::ffi::c_int {
        gctrj = gctrk;
        jempty = kempty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as ::core::ffi::c_int {
        gctri = gctrj;
        iempty = jempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as ::core::ffi::c_int {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    let mut eijk: ::core::ffi::c_double = 0.;
    let mut dijk: ::core::ffi::c_double = 0.;
    let mut aijk: ::core::ffi::c_double = 0.;
    let mut aiajrr: ::core::ffi::c_double = 0.;
    let mut aiakrr: ::core::ffi::c_double = 0.;
    let mut ajakrr: ::core::ffi::c_double = 0.;
    let mut rirk: [::core::ffi::c_double; 3] = [0.; 3];
    let mut rjrk: [::core::ffi::c_double; 3] = [0.; 3];
    rirk[0 as ::core::ffi::c_int as usize] = *ri.offset(0 as ::core::ffi::c_int as isize)
        - *rk.offset(0 as ::core::ffi::c_int as isize);
    rirk[1 as ::core::ffi::c_int as usize] = *ri.offset(1 as ::core::ffi::c_int as isize)
        - *rk.offset(1 as ::core::ffi::c_int as isize);
    rirk[2 as ::core::ffi::c_int as usize] = *ri.offset(2 as ::core::ffi::c_int as isize)
        - *rk.offset(2 as ::core::ffi::c_int as isize);
    rjrk[0 as ::core::ffi::c_int as usize] = *rj.offset(0 as ::core::ffi::c_int as isize)
        - *rk.offset(0 as ::core::ffi::c_int as isize);
    rjrk[1 as ::core::ffi::c_int as usize] = *rj.offset(1 as ::core::ffi::c_int as isize)
        - *rk.offset(1 as ::core::ffi::c_int as isize);
    rjrk[2 as ::core::ffi::c_int as usize] = *rj.offset(2 as ::core::ffi::c_int as isize)
        - *rk.offset(2 as ::core::ffi::c_int as isize);
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_ik: ::core::ffi::c_double = rirk[0 as ::core::ffi::c_int as usize]
        * rirk[0 as ::core::ffi::c_int as usize]
        + rirk[1 as ::core::ffi::c_int as usize] * rirk[1 as ::core::ffi::c_int as usize]
        + rirk[2 as ::core::ffi::c_int as usize]
            * rirk[2 as ::core::ffi::c_int as usize];
    let mut rr_jk: ::core::ffi::c_double = rjrk[0 as ::core::ffi::c_int as usize]
        * rjrk[0 as ::core::ffi::c_int as usize]
        + rjrk[1 as ::core::ffi::c_int as usize] * rjrk[1 as ::core::ffi::c_int as usize]
        + rjrk[2 as ::core::ffi::c_int as usize]
            * rjrk[2 as ::core::ffi::c_int as usize];
    kp = 0 as ::core::ffi::c_int;
    while kp < k_prim {
        (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as ::core::ffi::c_int {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *jempty = 1 as ::core::ffi::c_int;
        }
        jp = 0 as ::core::ffi::c_int;
        while jp < j_prim {
            (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as ::core::ffi::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as ::core::ffi::c_int;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as ::core::ffi::c_int;
            while ip < i_prim {
                (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > EXPCUTOFF as ::core::ffi::c_double) {
                    if i_ctr == 1 as ::core::ffi::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / (aijk * sqrt(aijk));
                    (*envs).fac[0 as ::core::ffi::c_int as usize] = dijk;
                    CINTg3c1e_ovlp(
                        g,
                        *ai.offset(ip as isize),
                        *aj.offset(jp as isize),
                        *ak.offset(kp as isize),
                        envs,
                    );
                    ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _),
                    >(
                        Some((*envs).f_gout.expect("non-null function pointer"))
                            .expect("non-null function pointer"),
                    )(gout, g, idx, envs, *gempty);
                    if i_ctr > 1 as ::core::ffi::c_int {
                        if *iempty != 0 {
                            CINTprim_to_ctr_0(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                len0,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        } else {
                            CINTprim_to_ctr_1(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                len0,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        }
                    }
                    *iempty = 0 as ::core::ffi::c_int;
                }
                ip += 1;
            }
            if *iempty == 0 {
                if j_ctr > 1 as ::core::ffi::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            leni,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            leni,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as ::core::ffi::c_int;
            }
            jp += 1;
        }
        if *jempty == 0 {
            if k_ctr > 1 as ::core::ffi::c_int {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                }
            }
            *kempty = 0 as ::core::ffi::c_int;
        }
        kp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
        *empty = 0 as ::core::ffi::c_int;
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c1e_nuc_loop_nopt(
    mut gctr: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
    mut fac: ::core::ffi::c_double,
    mut nuc_id: ::core::ffi::c_int,
    mut cache: *mut ::core::ffi::c_double,
    mut empty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut shls: *mut ::core::ffi::c_int = (*envs).shls;
    let mut atm: *mut ::core::ffi::c_int = (*envs).atm;
    let mut bas: *mut ::core::ffi::c_int = (*envs).bas;
    let mut env: *mut ::core::ffi::c_double = (*envs).env;
    let mut i_sh: ::core::ffi::c_int = *shls.offset(0 as ::core::ffi::c_int as isize);
    let mut j_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    let mut k_sh: ::core::ffi::c_int = *shls.offset(2 as ::core::ffi::c_int as isize);
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut ri: *mut ::core::ffi::c_double = (*envs).ri;
    let mut rj: *mut ::core::ffi::c_double = (*envs).rj;
    let mut rk: *mut ::core::ffi::c_double = (*envs).rk;
    let mut ai: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut aj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ak: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * k_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ci: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut cj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ck: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * k_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = 0;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 4] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut iempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize);
    let mut jempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut kempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(2 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut rys_empty: ::core::ffi::c_int = 0;
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = idx.offset(((*envs).nf * 3 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_double;
    CINTg2e_index_xyz(idx, envs);
    let mut non0ctri: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0ctrj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0ctrk: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxi: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxk: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + i_prim * i_ctr + j_prim * j_ctr + k_prim * k_ctr)
                as isize,
        ) as *mut ::core::ffi::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut cr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut t2: ::core::ffi::c_double = 0.;
    let mut tau: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut u: [::core::ffi::c_double; 32] = [0.; 32];
    let mut w: [::core::ffi::c_double; 32] = [0.; 32];
    let mut nc: ::core::ffi::c_int = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
    let mut len: size_t = leng
        .wrapping_add(lenk)
        .wrapping_add(lenj)
        .wrapping_add(leni)
        .wrapping_add(len0);
    let mut g: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    g = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut ::core::ffi::c_double = g.offset(leng as isize);
    let mut gout: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctri: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctrj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctrk: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as ::core::ffi::c_int {
        gctrj = gctrk;
        jempty = kempty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as ::core::ffi::c_int {
        gctri = gctrj;
        iempty = jempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as ::core::ffi::c_int {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    if nuc_id < 0 as ::core::ffi::c_int {
        cr = env.offset(PTR_RINV_ORIG as isize) as *mut ::core::ffi::c_double;
    } else {
        cr = env
            .offset(
                *atm.offset((ATM_SLOTS * nuc_id + 1 as ::core::ffi::c_int) as isize)
                    as isize,
            ) as *mut ::core::ffi::c_double;
    }
    let mut eijk: ::core::ffi::c_double = 0.;
    let mut dijk: ::core::ffi::c_double = 0.;
    let mut aijk: ::core::ffi::c_double = 0.;
    let mut aiajrr: ::core::ffi::c_double = 0.;
    let mut aiakrr: ::core::ffi::c_double = 0.;
    let mut ajakrr: ::core::ffi::c_double = 0.;
    let mut rirk: [::core::ffi::c_double; 3] = [0.; 3];
    let mut rjrk: [::core::ffi::c_double; 3] = [0.; 3];
    let mut rijk: [::core::ffi::c_double; 3] = [0.; 3];
    rirk[0 as ::core::ffi::c_int as usize] = *ri.offset(0 as ::core::ffi::c_int as isize)
        - *rk.offset(0 as ::core::ffi::c_int as isize);
    rirk[1 as ::core::ffi::c_int as usize] = *ri.offset(1 as ::core::ffi::c_int as isize)
        - *rk.offset(1 as ::core::ffi::c_int as isize);
    rirk[2 as ::core::ffi::c_int as usize] = *ri.offset(2 as ::core::ffi::c_int as isize)
        - *rk.offset(2 as ::core::ffi::c_int as isize);
    rjrk[0 as ::core::ffi::c_int as usize] = *rj.offset(0 as ::core::ffi::c_int as isize)
        - *rk.offset(0 as ::core::ffi::c_int as isize);
    rjrk[1 as ::core::ffi::c_int as usize] = *rj.offset(1 as ::core::ffi::c_int as isize)
        - *rk.offset(1 as ::core::ffi::c_int as isize);
    rjrk[2 as ::core::ffi::c_int as usize] = *rj.offset(2 as ::core::ffi::c_int as isize)
        - *rk.offset(2 as ::core::ffi::c_int as isize);
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_ik: ::core::ffi::c_double = rirk[0 as ::core::ffi::c_int as usize]
        * rirk[0 as ::core::ffi::c_int as usize]
        + rirk[1 as ::core::ffi::c_int as usize] * rirk[1 as ::core::ffi::c_int as usize]
        + rirk[2 as ::core::ffi::c_int as usize]
            * rirk[2 as ::core::ffi::c_int as usize];
    let mut rr_jk: ::core::ffi::c_double = rjrk[0 as ::core::ffi::c_int as usize]
        * rjrk[0 as ::core::ffi::c_int as usize]
        + rjrk[1 as ::core::ffi::c_int as usize] * rjrk[1 as ::core::ffi::c_int as usize]
        + rjrk[2 as ::core::ffi::c_int as usize]
            * rjrk[2 as ::core::ffi::c_int as usize];
    fac *= (*envs).common_factor;
    kp = 0 as ::core::ffi::c_int;
    while kp < k_prim {
        (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as ::core::ffi::c_int {
            fac1k = fac * *ck.offset(kp as isize);
        } else {
            fac1k = fac;
            *jempty = 1 as ::core::ffi::c_int;
        }
        jp = 0 as ::core::ffi::c_int;
        while jp < j_prim {
            (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as ::core::ffi::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as ::core::ffi::c_int;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as ::core::ffi::c_int;
            while ip < i_prim {
                (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > EXPCUTOFF as ::core::ffi::c_double) {
                    if i_ctr == 1 as ::core::ffi::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / aijk;
                    rijk[0 as ::core::ffi::c_int as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(0 as ::core::ffi::c_int as isize)
                        + *aj.offset(jp as isize)
                            * *rj.offset(0 as ::core::ffi::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(0 as ::core::ffi::c_int as isize)) / aijk;
                    rijk[1 as ::core::ffi::c_int as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(1 as ::core::ffi::c_int as isize)
                        + *aj.offset(jp as isize)
                            * *rj.offset(1 as ::core::ffi::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(1 as ::core::ffi::c_int as isize)) / aijk;
                    rijk[2 as ::core::ffi::c_int as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(2 as ::core::ffi::c_int as isize)
                        + *aj.offset(jp as isize)
                            * *rj.offset(2 as ::core::ffi::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(2 as ::core::ffi::c_int as isize)) / aijk;
                    tau = CINTnuc_mod(aijk, nuc_id, atm, env);
                    x = aijk
                        * CINTsquare_dist(
                            &raw mut rijk as *mut ::core::ffi::c_double,
                            cr,
                        ) * tau * tau;
                    CINTrys_roots(
                        (*envs).nrys_roots,
                        x,
                        &raw mut u as *mut ::core::ffi::c_double,
                        &raw mut w as *mut ::core::ffi::c_double,
                    );
                    rys_empty = *gempty;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*envs).nrys_roots {
                        t2 = u[i as usize]
                            / (1 as ::core::ffi::c_int as ::core::ffi::c_double
                                + u[i as usize]) * tau * tau;
                        (*envs).fac[0 as ::core::ffi::c_int as usize] = dijk
                            * w[i as usize] * tau;
                        CINTg3c1e_nuc(
                            g,
                            *ai.offset(ip as isize),
                            *aj.offset(jp as isize),
                            *ak.offset(kp as isize),
                            &raw mut rijk as *mut ::core::ffi::c_double,
                            cr,
                            t2,
                            envs,
                        );
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _),
                        >(
                            Some((*envs).f_gout.expect("non-null function pointer"))
                                .expect("non-null function pointer"),
                        )(gout, g, idx, envs, rys_empty);
                        rys_empty = 0 as ::core::ffi::c_int;
                        i += 1;
                    }
                    if i_ctr > 1 as ::core::ffi::c_int {
                        if *iempty != 0 {
                            CINTprim_to_ctr_0(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                ((*envs).nf * n_comp) as size_t,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        } else {
                            CINTprim_to_ctr_1(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                ((*envs).nf * n_comp) as size_t,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        }
                    }
                    *iempty = 0 as ::core::ffi::c_int;
                }
                ip += 1;
            }
            if *iempty == 0 {
                if j_ctr > 1 as ::core::ffi::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            ((*envs).nf * i_ctr * n_comp) as size_t,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            ((*envs).nf * i_ctr * n_comp) as size_t,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as ::core::ffi::c_int;
            }
            jp += 1;
        }
        if *jempty == 0 {
            if k_ctr > 1 as ::core::ffi::c_int {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                }
            }
            *kempty = 0 as ::core::ffi::c_int;
        }
        kp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
        *empty = 0 as ::core::ffi::c_int;
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c1e_drv(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
    mut f_e1_c2s: Option<unsafe extern "C" fn() -> ()>,
    mut int_type: ::core::ffi::c_int,
    mut is_ssc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut x_ctr: *mut ::core::ffi::c_int = &raw mut (*envs).x_ctr
        as *mut ::core::ffi::c_int;
    let mut nc: ::core::ffi::c_int = (*envs).nf
        * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
        * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
        * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    if out.is_null() {
        let mut bas: *mut ::core::ffi::c_int = (*envs).bas;
        let mut shls: *mut ::core::ffi::c_int = (*envs).shls;
        let mut i_prim: ::core::ffi::c_int = *bas
            .offset(
                (BAS_SLOTS * *shls.offset(0 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut j_prim: ::core::ffi::c_int = *bas
            .offset(
                (BAS_SLOTS * *shls.offset(1 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut k_prim: ::core::ffi::c_int = *bas
            .offset(
                (BAS_SLOTS * *shls.offset(2 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut pdata_size: ::core::ffi::c_int = i_prim
            * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
            + j_prim * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
            + k_prim * *x_ctr.offset(2 as ::core::ffi::c_int as isize)
            + (*envs).nf * 3 as ::core::ffi::c_int;
        let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
            * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
            as size_t;
        let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size: ::core::ffi::c_int = (if leng
            .wrapping_add(len0)
            .wrapping_add((nc * n_comp * 4 as ::core::ffi::c_int) as size_t)
            .wrapping_add(pdata_size as size_t)
            > (nc * n_comp + (*envs).nf * 3 as ::core::ffi::c_int) as size_t
        {
            leng.wrapping_add(len0)
                .wrapping_add((nc * n_comp * 4 as ::core::ffi::c_int) as size_t)
                .wrapping_add(pdata_size as size_t)
        } else {
            (nc * n_comp + (*envs).nf * 3 as ::core::ffi::c_int) as size_t
        }) as ::core::ffi::c_int;
        return cache_size;
    }
    let mut stack: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if cache.is_null() {
        let mut bas_0: *mut ::core::ffi::c_int = (*envs).bas;
        let mut shls_0: *mut ::core::ffi::c_int = (*envs).shls;
        let mut i_prim_0: ::core::ffi::c_int = *bas_0
            .offset(
                (BAS_SLOTS * *shls_0.offset(0 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut j_prim_0: ::core::ffi::c_int = *bas_0
            .offset(
                (BAS_SLOTS * *shls_0.offset(1 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut k_prim_0: ::core::ffi::c_int = *bas_0
            .offset(
                (BAS_SLOTS * *shls_0.offset(2 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut pdata_size_0: ::core::ffi::c_int = i_prim_0
            * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
            + j_prim_0 * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
            + k_prim_0 * *x_ctr.offset(2 as ::core::ffi::c_int as isize)
            + (*envs).nf * 3 as ::core::ffi::c_int;
        let mut leng_0: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
            * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
            as size_t;
        let mut len0_0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size_0: ::core::ffi::c_int = (if leng_0
            .wrapping_add(len0_0)
            .wrapping_add((nc * n_comp * 4 as ::core::ffi::c_int) as size_t)
            .wrapping_add(pdata_size_0 as size_t)
            > (nc * n_comp + (*envs).nf * 3 as ::core::ffi::c_int) as size_t
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add((nc * n_comp * 4 as ::core::ffi::c_int) as size_t)
                .wrapping_add(pdata_size_0 as size_t)
        } else {
            (nc * n_comp + (*envs).nf * 3 as ::core::ffi::c_int) as size_t
        }) as ::core::ffi::c_int;
        stack = malloc(
            (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                .wrapping_mul(cache_size_0 as size_t),
        ) as *mut ::core::ffi::c_double;
        cache = stack;
    }
    let mut gctr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut n: ::core::ffi::c_int = 0;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if int_type == INT1E_TYPE_OVLP {
        gctr = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_loop_nopt(gctr, envs, cache, &raw mut empty);
    } else if int_type == INT1E_TYPE_RINV {
        gctr = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_nuc_loop_nopt(
            gctr,
            envs,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            -(1 as ::core::ffi::c_int),
            cache,
            &raw mut empty,
        );
    } else {
        let mut atm: *mut ::core::ffi::c_int = (*envs).atm;
        let mut i: ::core::ffi::c_int = 0;
        let mut fac: ::core::ffi::c_double = 0.;
        let mut buf: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
            ::core::ffi::c_double,
        >();
        gctr = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        buf = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
        cache = buf.offset((nc * n_comp) as isize);
        n = 0 as ::core::ffi::c_int;
        while n < (*envs).natm {
            if *atm.offset((ATM_SLOTS * n + 0 as ::core::ffi::c_int) as isize)
                != 0 as ::core::ffi::c_int
            {
                fac = -abs(
                    *atm.offset((ATM_SLOTS * n + 0 as ::core::ffi::c_int) as isize),
                ) as ::core::ffi::c_double;
                CINT3c1e_nuc_loop_nopt(buf, envs, fac, n, cache, &raw mut empty);
            }
            n += 1;
        }
        cache = buf;
    }
    let mut counts: [::core::ffi::c_int; 4] = [0; 4];
    if f_e1_c2s
        == ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        )
        || f_e1_c2s
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    c2s_sph_3c2e1
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_double,
                            *mut ::core::ffi::c_double,
                            *mut ::core::ffi::c_int,
                            *mut CINTEnvVars,
                            *mut ::core::ffi::c_double,
                        ) -> (),
                ),
            )
    {
        counts[0 as ::core::ffi::c_int as usize] = ((*envs).i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(0 as ::core::ffi::c_int as isize);
        counts[1 as ::core::ffi::c_int as usize] = ((*envs).j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
        if is_ssc != 0 {
            counts[2 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed.nfk
                * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
        } else {
            counts[2 as ::core::ffi::c_int as usize] = ((*envs).k_l
                * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
        }
    } else {
        counts[0 as ::core::ffi::c_int as usize] = (*envs).nfi
            * *x_ctr.offset(0 as ::core::ffi::c_int as isize);
        counts[1 as ::core::ffi::c_int as usize] = (*envs).nfj
            * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
        counts[2 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
    }
    counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    if dims.is_null() {
        dims = &raw mut counts as *mut ::core::ffi::c_int;
    }
    let mut nout: ::core::ffi::c_int = *dims.offset(0 as ::core::ffi::c_int as isize)
        * *dims.offset(1 as ::core::ffi::c_int as isize)
        * *dims.offset(2 as ::core::ffi::c_int as isize);
    if empty == 0 {
        n = 0 as ::core::ffi::c_int;
        while n < n_comp {
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                Some(f_e1_c2s.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(
                out.offset((nout * n) as isize),
                gctr.offset((nc * n) as isize),
                dims,
                envs,
                cache,
            );
            n += 1;
        }
    } else {
        n = 0 as ::core::ffi::c_int;
        while n < n_comp {
            c2s_dset0(
                out.offset((nout * n) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            n += 1;
        }
    }
    if !stack.is_null() {
        free(stack as *mut ::core::ffi::c_void);
    }
    return (empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int3c1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    *opt = ::core::ptr::null_mut::<CINTOpt>();
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int3c1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int3c1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    *opt = ::core::ptr::null_mut::<CINTOpt>();
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int3c1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint3c1e_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int3c1e_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint3c1e_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int3c1e_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint3c1e_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int3c1e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint3c1e_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int3c1e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint3c1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int3c1e_optimizer(opt, atm, natm, bas, nbas, env);
}
pub const EXPCUTOFF: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
