#!/usr/bin/env python
"""对比 C 库 libcint 与 c2rust 转译 Rust 库在工作目录内针对实数(非复数)积分的数值."""
import os, sys, ctypes
import numpy as np
from pyscf import gto, lib

CRUST = os.path.abspath(os.path.join(os.path.dirname(__file__), 'out/target/release/librsint.so'))
CC    = os.path.abspath(os.path.join(os.path.dirname(__file__), '../libcint/build/libcint.so.6.1.3'))

def load(libpath):
    lib = ctypes.CDLL(libpath)
    return lib

def main():
    mol = gto.M(atom='H 0 0 0; H .2 .5 .8; H 1.9 2.1 .1',
                basis='sto-3g',
                spin=1,
                cart=True)
    atm = np.asarray(mol._atm, dtype=np.int32, order='C')
    bas = np.asarray(mol._bas, dtype=np.int32, order='C')
    env = np.asarray(mol._env, dtype=np.float64, order='C')
    natm, nbas = mol.natm, mol.nbas

    # 待验证的实数(非复数)积分函数 (无 _complex_/_spinor_)
    intors = ['1e_ovlp', '1e_kin', '1e_nuc',
              '2e_cart', '2e_cart_ip', '2c2e_cart', '3c2e_cart',
              '2e_sph', '3c2e_sph']

    rust = load(CRUST)
    cc   = load(CC)

    ok_all = True
    for name in intors:
        # 尝试各种后缀组合
        candidates = [f'cint{name}', f'cint{name}_cart', f'cint{name}_sph']
        cfn = None
        for c in candidates:
            if hasattr(cc, c) and hasattr(rust, c):
                cfn = c
                break
        if cfn is None:
            print(f"[skip] 未找到公共函数 {name}")
            continue
        cc_fn  = getattr(cc, cfn)
        rs_fn  = getattr(rust, cfn)
        cc_fn.restype = ctypes.c_int
        rs_fn.restype = ctypes.c_int
        # cint2e 需要 comp 参数判断，这里统一按 buf 大小循环调用 shell pair
        comp = 4 if '2e_cart' == name else 1
        # 简化：调用单 shell-pair 版本对比
        worst, n_fail = 0.0, 0
        for i in range(nbas):
            for j in range(nbas):
                # 从 pyscf 拿 ref buffer
                try:
                    ref = mol.intor(cfn, [i, j], comp=comp if False else None)
                except Exception:
                    ref = mol.intor(cfn, [i, j])
                buf_c  = np.zeros_like(ref)
                buf_rs = np.zeros_like(ref)
                cc_fn(buf_c.ctypes.data_as(ctypes.c_void_p),
                      (ctypes.c_int*2)(i,j),
                      atm.ctypes.data_as(ctypes.c_void_p), ctypes.c_int(natm),
                      bas.ctypes.data_as(ctypes.c_void_p), ctypes.c_int(nbas),
                      env.ctypes.data_as(ctypes.c_void_p),
                      lib.c_null_ptr())
                rs_fn(buf_rs.ctypes.data_as(ctypes.c_void_p),
                      (ctypes.c_int*2)(i,j),
                      atm.ctypes.data_as(ctypes.c_void_p), ctypes.c_int(natm),
                      bas.ctypes.data_as(ctypes.c_void_p), ctypes.c_int(nbas),
                      env.ctypes.data_as(ctypes.c_void_p),
                      lib.c_null_ptr())
                diff = np.abs(buf_c - buf_rs).max()
                if diff > worst:
                    worst = diff
                # 与总规模比较相对误差
        rel = worst / (np.abs(mol.intor(cfn)).max() + 1e-300)
        status = "OK " if worst < 1e-9 else "FAIL"
        if worst >= 1e-9:
            ok_all = False
        print(f"[{status}] {cfn:28s} max_abs_diff={worst:.3e}  rel={rel:.3e}")

    print("\n==== 汇总 ====  " + ("全部通过" if ok_all else "有失败项"))

if __name__ == '__main__':
    main()
