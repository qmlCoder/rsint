# AGENTS.md — 面向开发 Agent 的工程指引

本文件面向在此仓库中工作的编码/开发 Agent。详细背景见 [README.md](./README.md)。
本项目是将 C 库 libcint 通过 c2rust 机械转译为 Rust 的工程。

## 重要约定（遵守）

- **转译产物为"能跑的 C 版本"，不是地道 Rust**：大量 `unsafe`、裸指针、可变全局
  `static mut`、`extern "C"` 函数由 c2rust 生成。除非明确要求"人工重构"，否则不要把
  `work/out/src/*.rs` 改成地道 Rust 风格。
- **不要直接修改 `libcint/` 下的原始 C 源码**。它是参照基准与测试基准。任何为了
  转译而做的改动（如 quadmath→double）只允许发生在 `src_work/` 中。
- **修改转译产物前先跑数值对比**：必须保证 `compare2.py` 全部 `PASS`。任何会破坏
  数值等价性的改动都不可接受。

## 工具链速查

| 用途 | 命令 |
|------|------|
| 转译 | `cargo +nightly-2025-02-01 install c2rust --locked` |
| 生成 C 参照库 | `cd libcint && cmake -S . -B build && cmake --build build -j$(nproc)` |
| 重新转译 | `c2rust transpile work/compile_commands.json -e -o work/out` |
| 构建 Rust | `cd work/out && cargo build --release` |
| 数值验证 | `cd work && python3 compare2.py` |

- **Rust 工具链必须用 `nightly-2025-02-01`**（转译产物依赖 `raw_ref_op`、
  `extern_types`、`label_break_value`）。`rust-toolchain.toml` 已固定。
- 最新 stable/nightly 无法编译转译产物（feature 与 f128 依赖均已不兼容）。
- c2rust 版本固定为 **0.22.1**；不要用 GitHub master 分支（其 clang AST exporter
  依赖与当前编译环境不兼容）。

## 目录地图

```
libcint/      C 原始仓库 + build/libcint.so（数值基准，勿改）
src_work/    转译输入源副本（已做 quadmath/long double→double 替换）
work/
  compile_commands.json   转译编译数据库
  compare2.py             C vs Rust 数值对比脚本（唯一验证入口）
  out/                    转译产物（cargo 工程，main 工作区）
    lib.rs                模块组装
    helpers.rs            人工补充符号（CINTsquare_dist）
    src/                  41 个 .rs 模块，autocode/ 为 Lisp 生成的积分代码
    target/release/librsint.so  Rust cdylib 产物
rust_out/ rust_out_e/ rust_test/ build_cc_work.json  早期试验，可清理
```

## 转译 pipeline（重新生成时）

1. `src_work/` 中已完成替换：
   - `__float128`/`_Float128`/`long double` → `double`
   - `erfcq`→`erfc`、`erfq`→`erf`、`expq`→`exp`、`sqrtq`→`sqrt`、`fabsq`→`fabs`
   - 去掉 `q`/`l` 数字后缀字面量
2. `build/src/cint_config.h` 需禁用 `HAVE_QUADMATH_H`、`HAVE_EXPL`、`HAVE_SQRTL`、
   `HAVE_FABSL`（避免 f128/`sqrtl` 依赖）。
3. `work/compile_commands.json` 指向 `src_work/` 路径并排除 `misc.c`、
   禁用 quadmath 宏。
4. 转译后从 `Cargo.toml` 移除 `f128 = "0.2"` 依赖。
5. 若出现未定义符号，检查并用 `nm -D --undefined-only` 定位，在 `helpers.rs`
   补充（当前只需 `CINTsquare_dist`）。

## 已知限制（agent 应知晓）

- **复数/spinor 无法自动转译**：c2rust 0.22.1 不支持 `double complex`，479 个复数
  函数被跳过（见 `README.md`）。不要浪费时间尝试用 c2rust 转它们。
- **`misc.c` 未转译**（含 `_Complex_I`）。实数路径不依赖它；`CINTsquare_dist`
  已在 helpers.rs 补齐。
- 转译产物有 ~531 个 warning（如重复 extern 声明），正常，不必清理。

## 验证规范

- **任何改动后**必须跑 `cd work && python3 compare2.py`。
- 判据：所有列出的积分函数 `PASS` 且 `worst_abs_err` ≤ `1e-9`。
- 添加新积分验证时：pyscf `intor_by_shell` 取 buffer 形状，shell-pair API
  `fn(buf, shls[2], atm, natm, bas, nbas, env, cintopt)`，restype 设 `c_int`。
