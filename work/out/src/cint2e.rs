extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn CINTcgto_spinor(
        bas_id: ::core::ffi::c_int,
        bas: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn CINTOpt_log_max_pgto_coeff(
        log_maxc: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
    );
    fn CINTOpt_non0coeff_byshell(
        sortedidx: *mut ::core::ffi::c_int,
        non0ctr: *mut ::core::ffi::c_int,
        ci: *mut ::core::ffi::c_double,
        iprim: ::core::ffi::c_int,
        ictr: ::core::ffi::c_int,
    );
    fn CINTset_pairdata(
        pairdata: *mut PairData,
        ai: *mut ::core::ffi::c_double,
        aj: *mut ::core::ffi::c_double,
        ri: *mut ::core::ffi::c_double,
        rj: *mut ::core::ffi::c_double,
        log_maxci: *mut ::core::ffi::c_double,
        log_maxcj: *mut ::core::ffi::c_double,
        li_ceil: ::core::ffi::c_int,
        lj_ceil: ::core::ffi::c_int,
        iprim: ::core::ffi::c_int,
        jprim: ::core::ffi::c_int,
        rr_ij: ::core::ffi::c_double,
        expcutoff: ::core::ffi::c_double,
        env: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn CINTall_2e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
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
    fn c2s_sph_2e1(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_cart_2e1(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_sf_2e1(
        opij: *mut ::core::ffi::c_double,
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
    fn CINTg2e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *const CINTEnvVars);
    fn CINTinit_int2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
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
pub type uintptr_t = usize;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const PTR_RANGE_OMEGA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LMAX1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINT2e_loop_nopt(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
    let mut rk: *mut ::core::ffi::c_double = (*envs).rk;
    let mut rl: *mut ::core::ffi::c_double = (*envs).c2rust_unnamed_1.rl;
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut log_maxci: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut log_maxcj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut log_maxck: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut log_maxcl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut pdata_base: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = log_maxci.offset((i_prim + j_prim + k_prim + l_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut ::core::ffi::c_double;
    log_maxcj = log_maxci.offset(i_prim as isize);
    log_maxck = log_maxcj.offset(j_prim as isize);
    log_maxcl = log_maxck.offset(k_prim as isize);
    CINTOpt_log_max_pgto_coeff(log_maxci, ci, i_prim, i_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcj, cj, j_prim, j_ctr);
    if CINTset_pairdata(
        pdata_base,
        ai,
        aj,
        (*envs).ri,
        (*envs).rj,
        log_maxci,
        log_maxcj,
        (*envs).li_ceil,
        (*envs).lj_ceil,
        i_prim,
        j_prim,
        rr_ij,
        expcutoff,
        env,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    CINTOpt_log_max_pgto_coeff(log_maxck, ck, k_prim, k_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcl, cl, l_prim, l_ctr);
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut akl: ::core::ffi::c_double = 0.;
    let mut ekl: ::core::ffi::c_double = 0.;
    let mut expijkl: ::core::ffi::c_double = 0.;
    let mut ccekl: ::core::ffi::c_double = 0.;
    let mut log_rr_kl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    let mut rkl: [::core::ffi::c_double; 3] = [0.; 3];
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    akl = *ak.offset((k_prim - 1 as ::core::ffi::c_int) as isize)
        + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
    log_rr_kl = 1.7f64 - 1.5f64 * log(akl);
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if (*envs).rys_order > 1 as ::core::ffi::c_int {
            let mut r_guess: ::core::ffi::c_double = 8.0f64;
            let mut omega2: ::core::ffi::c_double = omega * omega;
            let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
            if lij > 0 as ::core::ffi::c_int {
                let mut aij: ::core::ffi::c_double = *ai
                    .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                    + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
                let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
                let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
                expcutoff
                    += lij as ::core::ffi::c_double
                        * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
            }
            if lkl > 0 as ::core::ffi::c_int {
                let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
                log_rr_kl
                    += lkl as ::core::ffi::c_double
                        * log(sqrt(rr_kl) + theta_0 * r_guess + 1.0f64);
            }
        }
    } else if lkl > 0 as ::core::ffi::c_int {
        log_rr_kl += lkl as ::core::ffi::c_double * log(sqrt(rr_kl) + 1.0f64);
    }
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
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
    let mut non0ctrl: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
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
    let mut non0idxl: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + l_prim + i_prim * i_ctr + j_prim * j_ctr
                + k_prim * k_ctr + l_prim * l_ctr) as isize,
        ) as *mut ::core::ffi::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0ctrl = non0ctrk.offset(k_prim as isize);
    non0idxi = non0ctrl.offset(l_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    non0idxl = non0idxk.offset((k_prim * k_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    CINTOpt_non0coeff_byshell(non0idxl, non0ctrl, cl, l_prim, l_ctr);
    let mut nc: ::core::ffi::c_int = i_ctr * j_ctr * k_ctr * l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenl: size_t = nf.wrapping_mul(nc as size_t).wrapping_mul(n_comp as size_t);
    let mut lenk: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(j_ctr as size_t)
        .wrapping_mul(k_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut lenj: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(j_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng
        .wrapping_add(lenl)
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
    let mut gctrl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    if l_ctr == 1 as ::core::ffi::c_int {
        gctrk = gctrl;
        kempty = lempty;
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
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        if l_ctr == 1 as ::core::ffi::c_int {
            fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        } else {
            fac1l = (*envs).common_factor;
            *kempty = 1 as ::core::ffi::c_int;
        }
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            akl = *ak.offset(kp as isize) + *al.offset(lp as isize);
            ekl = rr_kl * *ak.offset(kp as isize) * *al.offset(lp as isize) / akl;
            ccekl = ekl - log_rr_kl - *log_maxck.offset(kp as isize)
                - *log_maxcl.offset(lp as isize);
            if !(ccekl > expcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                rkl[0 as ::core::ffi::c_int as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(0 as ::core::ffi::c_int as isize)
                    + *al.offset(lp as isize)
                        * *rl.offset(0 as ::core::ffi::c_int as isize)) / akl;
                rkl[1 as ::core::ffi::c_int as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(1 as ::core::ffi::c_int as isize)
                    + *al.offset(lp as isize)
                        * *rl.offset(1 as ::core::ffi::c_int as isize)) / akl;
                rkl[2 as ::core::ffi::c_int as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(2 as ::core::ffi::c_int as isize)
                    + *al.offset(lp as isize)
                        * *rl.offset(2 as ::core::ffi::c_int as isize)) / akl;
                eijcutoff = expcutoff - ccekl;
                ekl = exp(-ekl);
                if k_ctr == 1 as ::core::ffi::c_int {
                    fac1k = fac1l * *ck.offset(kp as isize);
                } else {
                    fac1k = fac1l;
                    *jempty = 1 as ::core::ffi::c_int;
                }
                pdata_ij = pdata_base;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    if j_ctr == 1 as ::core::ffi::c_int {
                        fac1j = fac1k * *cj.offset(jp as isize);
                    } else {
                        fac1j = fac1k;
                        *iempty = 1 as ::core::ffi::c_int;
                    }
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            expijkl = (*pdata_ij).eij * ekl;
                            if i_ctr == 1 as ::core::ffi::c_int {
                                fac1i = fac1j * *ci.offset(ip as isize) * expijkl;
                            } else {
                                fac1i = fac1j * expijkl;
                            }
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(
                                g,
                                rij,
                                &raw mut rkl as *mut ::core::ffi::c_double,
                                cutoff,
                                envs,
                            ) != 0
                            {
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
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
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
            }
            kp += 1;
        }
        if *kempty == 0 {
            if l_ctr > 1 as ::core::ffi::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as ::core::ffi::c_int;
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_1111_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng.wrapping_add(len0);
    let mut gout: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut g: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    g = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = g.offset(len as isize);
    if n_comp == 1 as ::core::ffi::c_int {
        gout = gctr;
        gempty = empty;
    } else {
        gout = g.offset(leng as isize);
    }
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    Some((*envs).f_gout.expect("non-null function pointer"))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *gempty);
                                *gempty = 0 as ::core::ffi::c_int;
                            }
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
                    }
                    jp += 1;
                }
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *gempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_n111_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = i_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng.wrapping_add(leni).wrapping_add(len0);
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
    if n_comp == 1 as ::core::ffi::c_int {
        gctri = gctr;
        iempty = empty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            if !((*pdata_ij).cceij > eijcutoff) {
                                (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                    .offset(ip as isize);
                                expij = (*pdata_ij).eij;
                                rij = &raw mut (*pdata_ij).rij
                                    as *mut ::core::ffi::c_double;
                                cutoff = eijcutoff - (*pdata_ij).cceij;
                                fac1i = fac1j * expij * expkl;
                                (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                                if ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _) -> ::core::ffi::c_int,
                                >(
                                    Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                        .expect("non-null function pointer"),
                                )(g, rij, rkl, cutoff, envs) != 0
                                {
                                    ::core::mem::transmute::<
                                        _,
                                        fn(_, _, _, _, _),
                                    >(
                                        Some((*envs).f_gout.expect("non-null function pointer"))
                                            .expect("non-null function pointer"),
                                    )(gout, g, idx, envs, 1 as ::core::ffi::c_int);
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
                            }
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
                    }
                    jp += 1;
                }
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *iempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_1n11_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = j_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenj: size_t = nf
        .wrapping_mul(j_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng.wrapping_add(lenj).wrapping_add(len0);
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
    let mut gctrj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrj = gctr;
        jempty = empty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    fac1j = fac1k;
                    *iempty = 1 as ::core::ffi::c_int;
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    Some((*envs).f_gout.expect("non-null function pointer"))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *iempty);
                                *iempty = 0 as ::core::ffi::c_int;
                            }
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
                    }
                    if *iempty == 0 {
                        if j_ctr > 1 as ::core::ffi::c_int {
                            if *jempty != 0 {
                                CINTprim_to_ctr_0(
                                    gctrj,
                                    gout,
                                    cj.offset(jp as isize),
                                    len0,
                                    j_prim,
                                    j_ctr,
                                    *non0ctrj.offset(jp as isize),
                                    non0idxj.offset((jp * j_ctr) as isize),
                                );
                            } else {
                                CINTprim_to_ctr_1(
                                    gctrj,
                                    gout,
                                    cj.offset(jp as isize),
                                    len0,
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
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *jempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_11n1_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenk: size_t = nf
        .wrapping_mul(k_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng.wrapping_add(lenk).wrapping_add(len0);
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
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                fac1k = fac1l;
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                *jempty = 1 as ::core::ffi::c_int;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    Some((*envs).f_gout.expect("non-null function pointer"))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *jempty);
                                *jempty = 0 as ::core::ffi::c_int;
                            }
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
                    }
                    jp += 1;
                }
                if *jempty == 0 {
                    if k_ctr > 1 as ::core::ffi::c_int {
                        if *kempty != 0 {
                            CINTprim_to_ctr_0(
                                gctrk,
                                gout,
                                ck.offset(kp as isize),
                                len0,
                                k_prim,
                                k_ctr,
                                *non0ctrk.offset(kp as isize),
                                non0idxk.offset((kp * k_ctr) as isize),
                            );
                        } else {
                            CINTprim_to_ctr_1(
                                gctrk,
                                gout,
                                ck.offset(kp as isize),
                                len0,
                                k_prim,
                                k_ctr,
                                *non0ctrk.offset(kp as isize),
                                non0idxk.offset((kp * k_ctr) as isize),
                            );
                        }
                    }
                    *kempty = 0 as ::core::ffi::c_int;
                }
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_111n_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenl: size_t = nf
        .wrapping_mul(l_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng.wrapping_add(lenl).wrapping_add(len0);
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
    let mut gctrl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor;
        *kempty = 1 as ::core::ffi::c_int;
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    Some((*envs).f_gout.expect("non-null function pointer"))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *kempty);
                                *kempty = 0 as ::core::ffi::c_int;
                            }
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
                    }
                    jp += 1;
                }
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        if *kempty == 0 {
            if l_ctr > 1 as ::core::ffi::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gout,
                        cl.offset(lp as isize),
                        len0,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gout,
                        cl.offset(lp as isize),
                        len0,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as ::core::ffi::c_int;
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_loop(
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
    let mut l_sh: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !(*opt).pairdata.is_null()
        && (*(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize)
            == NOVALUE as *mut PairData
            || *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize)
                == NOVALUE as *mut PairData)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut k_ctr: ::core::ffi::c_int = (*envs).x_ctr[2 as ::core::ffi::c_int as usize];
    let mut l_ctr: ::core::ffi::c_int = (*envs).x_ctr[3 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut k_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * k_sh + 2 as ::core::ffi::c_int) as isize);
    let mut l_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * l_sh + 2 as ::core::ffi::c_int) as isize);
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
    let mut al: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 5 as ::core::ffi::c_int) as isize) as isize,
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
    let mut cl: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * l_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut rr_ij: ::core::ffi::c_double = (*envs).rirj[0 as ::core::ffi::c_int as usize]
        * (*envs).rirj[0 as ::core::ffi::c_int as usize]
        + (*envs).rirj[1 as ::core::ffi::c_int as usize]
            * (*envs).rirj[1 as ::core::ffi::c_int as usize]
        + (*envs).rirj[2 as ::core::ffi::c_int as usize]
            * (*envs).rirj[2 as ::core::ffi::c_int as usize];
    let mut rr_kl: ::core::ffi::c_double = (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        * (*envs).rkrl[0 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[1 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[1 as ::core::ffi::c_int as usize]
        + (*envs).rkrl[2 as ::core::ffi::c_int as usize]
            * (*envs).rkrl[2 as ::core::ffi::c_int as usize];
    let mut _pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut _pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_kl: *mut PairData = ::core::ptr::null_mut::<PairData>();
    if !(*opt).pairdata.is_null() {
        _pdata_ij = *(*opt).pairdata.offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *(*opt).pairdata.offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(i_sh as isize);
        let mut log_maxcj: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t)
            .wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut ::core::ffi::c_double;
        if CINTset_pairdata(
            _pdata_ij,
            ai,
            aj,
            (*envs).ri,
            (*envs).rj,
            log_maxci,
            log_maxcj,
            (*envs).li_ceil,
            (*envs).lj_ceil,
            i_prim,
            j_prim,
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut log_maxck: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(k_sh as isize);
        let mut log_maxcl: *mut ::core::ffi::c_double = *(*opt)
            .log_max_coeff
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut fac1k: ::core::ffi::c_double = 0.;
    let mut fac1l: ::core::ffi::c_double = 0.;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let mut lp: ::core::ffi::c_int = 0;
    let mut _empty: [::core::ffi::c_int; 5] = [
        1 as ::core::ffi::c_int,
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
    let mut lempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(3 as ::core::ffi::c_int as isize);
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut _empty
        as *mut ::core::ffi::c_int)
        .offset(4 as ::core::ffi::c_int as isize);
    let mut non0ctri: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(i_sh as isize);
    let mut non0ctrj: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(j_sh as isize);
    let mut non0ctrk: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(k_sh as isize);
    let mut non0ctrl: *mut ::core::ffi::c_int = *(*opt).non0ctr.offset(l_sh as isize);
    let mut non0idxi: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(i_sh as isize);
    let mut non0idxj: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(j_sh as isize);
    let mut non0idxk: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(k_sh as isize);
    let mut non0idxl: *mut ::core::ffi::c_int = *(*opt).sortedidx.offset(l_sh as isize);
    let mut expij: ::core::ffi::c_double = 0.;
    let mut expkl: ::core::ffi::c_double = 0.;
    let mut eijcutoff: ::core::ffi::c_double = 0.;
    let mut eklcutoff: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rkl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut idx: *mut ::core::ffi::c_int = *(*opt)
        .index_xyz_array
        .offset(
            ((*envs).i_l * LMAX1 * LMAX1 * LMAX1 + (*envs).j_l * LMAX1 * LMAX1
                + (*envs).k_l * LMAX1 + (*envs).l_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
            & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as size_t) as isize)
            as *mut ::core::ffi::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*envs).rys_order > 1 as ::core::ffi::c_int
    {
        let mut r_guess: ::core::ffi::c_double = 8.0f64;
        let mut omega2: ::core::ffi::c_double = omega * omega;
        let mut lij: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        let mut lkl: ::core::ffi::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
        if lij > 0 as ::core::ffi::c_int {
            let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
            let mut aij: ::core::ffi::c_double = *ai
                .offset((i_prim - 1 as ::core::ffi::c_int) as isize)
                + *aj.offset((j_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as ::core::ffi::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if lkl > 0 as ::core::ffi::c_int {
            let mut dist_kl: ::core::ffi::c_double = sqrt(rr_kl);
            let mut akl: ::core::ffi::c_double = *ak
                .offset((k_prim - 1 as ::core::ffi::c_int) as isize)
                + *al.offset((l_prim - 1 as ::core::ffi::c_int) as isize);
            let mut theta_0: ::core::ffi::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as ::core::ffi::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: ::core::ffi::c_int = i_ctr * j_ctr * k_ctr * l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut lenl: size_t = nf.wrapping_mul(nc as size_t).wrapping_mul(n_comp as size_t);
    let mut lenk: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(j_ctr as size_t)
        .wrapping_mul(k_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut lenj: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(j_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as size_t)
        .wrapping_mul(n_comp as size_t);
    let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
    let mut len: size_t = leng
        .wrapping_add(lenl)
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
    let mut gctrl: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    if l_ctr == 1 as ::core::ffi::c_int {
        gctrk = gctrl;
        kempty = lempty;
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
    pdata_kl = _pdata_kl;
    lp = 0 as ::core::ffi::c_int;
    while lp < l_prim {
        (*envs).al[0 as ::core::ffi::c_int as usize] = *al.offset(lp as isize);
        if l_ctr == 1 as ::core::ffi::c_int {
            fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        } else {
            fac1l = (*envs).common_factor;
            *kempty = 1 as ::core::ffi::c_int;
        }
        kp = 0 as ::core::ffi::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as ::core::ffi::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = &raw mut (*pdata_kl).rij as *mut ::core::ffi::c_double;
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                if k_ctr == 1 as ::core::ffi::c_int {
                    fac1k = fac1l * *ck.offset(kp as isize);
                } else {
                    fac1k = fac1l;
                    *jempty = 1 as ::core::ffi::c_int;
                }
                pdata_ij = _pdata_ij;
                jp = 0 as ::core::ffi::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj
                        .offset(jp as isize);
                    if j_ctr == 1 as ::core::ffi::c_int {
                        fac1j = fac1k * *cj.offset(jp as isize);
                    } else {
                        fac1j = fac1k;
                        *iempty = 1 as ::core::ffi::c_int;
                    }
                    ip = 0 as ::core::ffi::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                                .offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            if i_ctr == 1 as ::core::ffi::c_int {
                                fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            } else {
                                fac1i = fac1j * expij * expkl;
                            }
                            (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> ::core::ffi::c_int,
                            >(
                                Some((*envs).f_g0_2e.expect("non-null function pointer"))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
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
                        }
                        ip += 1;
                        pdata_ij = pdata_ij.offset(1);
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
            }
            kp += 1;
            pdata_kl = pdata_kl.offset(1);
        }
        if *kempty == 0 {
            if l_ctr > 1 as ::core::ffi::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as ::core::ffi::c_int;
        }
        lp += 1;
    }
    if n_comp > 1 as ::core::ffi::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
            *empty = 0 as ::core::ffi::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as size_t) as ::core::ffi::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as ::core::ffi::c_int;
}
static mut CINTf_2e_loop: [Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_double,
        *mut CINTEnvVars,
        *mut ::core::ffi::c_double,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>; 16] = unsafe {
    [
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_n111_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_1n11_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_11n1_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_111n_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            CINT2e_1111_loop
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CINT2e_drv(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
    mut f_c2s: Option<unsafe extern "C" fn() -> ()>,
) -> ::core::ffi::c_int {
    let mut x_ctr: *mut ::core::ffi::c_int = &raw mut (*envs).x_ctr
        as *mut ::core::ffi::c_int;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut nc: size_t = nf
        .wrapping_mul(*x_ctr.offset(0 as ::core::ffi::c_int as isize) as size_t)
        .wrapping_mul(*x_ctr.offset(1 as ::core::ffi::c_int as isize) as size_t)
        .wrapping_mul(*x_ctr.offset(2 as ::core::ffi::c_int as isize) as size_t)
        .wrapping_mul(*x_ctr.offset(3 as ::core::ffi::c_int as isize) as size_t);
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
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
        let mut l_prim: ::core::ffi::c_int = *bas
            .offset(
                (BAS_SLOTS * *shls.offset(3 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut pdata_size: size_t = (((i_prim * j_prim + k_prim * l_prim)
            * 5 as ::core::ffi::c_int
            + i_prim * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
            + j_prim * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
            + k_prim * *x_ctr.offset(2 as ::core::ffi::c_int as isize)
            + l_prim * *x_ctr.offset(3 as ::core::ffi::c_int as isize)
            + (i_prim + j_prim + k_prim + l_prim) * 2 as ::core::ffi::c_int) as size_t)
            .wrapping_add(nf.wrapping_mul(3 as size_t));
        let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
            * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
            as size_t;
        let mut len0: size_t = nf.wrapping_mul(n_comp as size_t);
        let mut cache_size: size_t = if leng
            .wrapping_add(len0)
            .wrapping_add(nc.wrapping_mul(n_comp as size_t).wrapping_mul(3 as size_t))
            .wrapping_add(pdata_size)
            > nc
                .wrapping_mul(n_comp as size_t)
                .wrapping_add(nf.wrapping_mul(4 as size_t))
        {
            leng.wrapping_add(len0)
                .wrapping_add(
                    nc.wrapping_mul(n_comp as size_t).wrapping_mul(3 as size_t),
                )
                .wrapping_add(pdata_size)
        } else {
            nc.wrapping_mul(n_comp as size_t).wrapping_add(nf.wrapping_mul(4 as size_t))
        };
        if cache_size >= INT32_MAX as size_t {
            fprintf(
                stderr,
                b"CINT2e_drv cache_size overflow: cache_size %zu > %d, nf %zu, nc %zu, n_comp %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                cache_size,
                INT32_MAX,
                nf,
                nc,
                n_comp,
            );
            cache_size = 0 as size_t;
        }
        return cache_size as ::core::ffi::c_int;
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
        let mut l_prim_0: ::core::ffi::c_int = *bas_0
            .offset(
                (BAS_SLOTS * *shls_0.offset(3 as ::core::ffi::c_int as isize)
                    + 2 as ::core::ffi::c_int) as isize,
            );
        let mut pdata_size_0: size_t = (((i_prim_0 * j_prim_0 + k_prim_0 * l_prim_0)
            * 5 as ::core::ffi::c_int
            + i_prim_0 * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
            + j_prim_0 * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
            + k_prim_0 * *x_ctr.offset(2 as ::core::ffi::c_int as isize)
            + l_prim_0 * *x_ctr.offset(3 as ::core::ffi::c_int as isize)
            + (i_prim_0 + j_prim_0 + k_prim_0 + l_prim_0) * 2 as ::core::ffi::c_int)
            as size_t)
            .wrapping_add(nf.wrapping_mul(3 as size_t));
        let mut leng_0: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
            * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
            as size_t;
        let mut len0_0: size_t = nf.wrapping_mul(n_comp as size_t);
        let mut cache_size_0: size_t = if leng_0
            .wrapping_add(len0_0)
            .wrapping_add(nc.wrapping_mul(n_comp as size_t).wrapping_mul(3 as size_t))
            .wrapping_add(pdata_size_0)
            > nc
                .wrapping_mul(n_comp as size_t)
                .wrapping_add(nf.wrapping_mul(4 as size_t))
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add(
                    nc.wrapping_mul(n_comp as size_t).wrapping_mul(3 as size_t),
                )
                .wrapping_add(pdata_size_0)
        } else {
            nc.wrapping_mul(n_comp as size_t).wrapping_add(nf.wrapping_mul(4 as size_t))
        };
        stack = malloc(
            (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                .wrapping_mul(cache_size_0),
        ) as *mut ::core::ffi::c_double;
        cache = stack;
    }
    let mut gctr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    gctr = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = gctr.offset(nc.wrapping_mul(n_comp as size_t) as isize);
    let mut n: ::core::ffi::c_int = 0;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !opt.is_null() {
        (*envs).opt = opt;
        n = (((*x_ctr.offset(0 as ::core::ffi::c_int as isize)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int)
            + (((*x_ctr.offset(1 as ::core::ffi::c_int as isize)
                == 1 as ::core::ffi::c_int) as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)
            + (((*x_ctr.offset(2 as ::core::ffi::c_int as isize)
                == 1 as ::core::ffi::c_int) as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
            + (*x_ctr.offset(3 as ::core::ffi::c_int as isize)
                == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        CINTf_2e_loop[n as usize]
            .expect("non-null function pointer")(gctr, envs, cache, &raw mut empty);
    } else {
        CINT2e_loop_nopt(gctr, envs, cache, &raw mut empty);
    }
    let mut counts: [::core::ffi::c_int; 4] = [0; 4];
    if f_c2s
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
                c2s_sph_2e1
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
        counts[2 as ::core::ffi::c_int as usize] = ((*envs).k_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
        counts[3 as ::core::ffi::c_int as usize] = ((*envs).l_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(3 as ::core::ffi::c_int as isize);
    } else {
        counts[0 as ::core::ffi::c_int as usize] = (*envs).nfi
            * *x_ctr.offset(0 as ::core::ffi::c_int as isize);
        counts[1 as ::core::ffi::c_int as usize] = (*envs).nfj
            * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
        counts[2 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(2 as ::core::ffi::c_int as isize);
        counts[3 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed_0.nfl
            * *x_ctr.offset(3 as ::core::ffi::c_int as isize);
    }
    if dims.is_null() {
        dims = &raw mut counts as *mut ::core::ffi::c_int;
    }
    let mut nout: ::core::ffi::c_int = *dims.offset(0 as ::core::ffi::c_int as isize)
        * *dims.offset(1 as ::core::ffi::c_int as isize)
        * *dims.offset(2 as ::core::ffi::c_int as isize)
        * *dims.offset(3 as ::core::ffi::c_int as isize);
    if empty == 0 {
        n = 0 as ::core::ffi::c_int;
        while n < n_comp {
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                Some(f_c2s.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(
                out.offset((nout * n) as isize),
                gctr.offset(nc.wrapping_mul(n as size_t) as isize),
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
pub unsafe extern "C" fn CINTgout2e(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_double = 0.;
    if gout_empty != 0 {
        match (*envs).nrys_roots {
            1 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            2 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            3 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            4 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 3 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            5 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 4 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            6 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 5 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            7 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 5 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 6 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 6 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 6 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            8 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize) = *g.offset(ix as isize)
                        * *g.offset(iy as isize) * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 5 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 6 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 6 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 6 as ::core::ffi::c_int) as isize)
                        + *g.offset((ix + 7 as ::core::ffi::c_int) as isize)
                            * *g.offset((iy + 7 as ::core::ffi::c_int) as isize)
                            * *g.offset((iz + 7 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            _ => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    s = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*envs).nrys_roots {
                        s
                            += *g.offset((ix + i) as isize)
                                * *g.offset((iy + i) as isize)
                                * *g.offset((iz + i) as isize);
                        i += 1;
                    }
                    *gout.offset(n as isize) = s;
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
        }
    } else {
        match (*envs).nrys_roots {
            1 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            2 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            3 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            4 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 3 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            5 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 4 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            6 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 5 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            7 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 5 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 6 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 6 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 6 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            8 => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 1 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 1 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 2 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 2 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 3 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 3 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 4 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 4 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 5 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 5 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 6 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 6 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 6 as ::core::ffi::c_int) as isize)
                            + *g.offset((ix + 7 as ::core::ffi::c_int) as isize)
                                * *g.offset((iy + 7 as ::core::ffi::c_int) as isize)
                                * *g.offset((iz + 7 as ::core::ffi::c_int) as isize);
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
            _ => {
                n = 0 as ::core::ffi::c_int;
                while n < nf {
                    ix = *idx.offset(0 as ::core::ffi::c_int as isize);
                    iy = *idx.offset(1 as ::core::ffi::c_int as isize);
                    iz = *idx.offset(2 as ::core::ffi::c_int as isize);
                    s = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*envs).nrys_roots {
                        s
                            += *g.offset((ix + i) as isize)
                                * *g.offset((iy + i) as isize)
                                * *g.offset((iz + i) as isize);
                        i += 1;
                    }
                    *gout.offset(n as isize) += s;
                    n += 1;
                    idx = idx.offset(3 as ::core::ffi::c_int as isize);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn int2e_sph(
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_sph_2e1
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
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
    CINTall_2e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_cart(
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_cart_2e1
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_cart(
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
pub unsafe extern "C" fn cint2e_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_sph(
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
pub unsafe extern "C" fn cint2e_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
pub const NOVALUE: *mut ::core::ffi::c_void = 0xffffffffffffffff as ::core::ffi::c_ulong
    as *mut ::core::ffi::c_void;
