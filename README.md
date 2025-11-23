# Rust Mono - Rust 多模块项目（静态链接 vs 动态库对比）

使用 **Cargo Workspace** 管理的多模块项目，对比静态链接与动态链接的实际差异。

## 项目结构

```
rust-mono/
├── Cargo.toml            # Cargo Workspace 配置
├── packages/             # 库模块
│   ├── common/          # 通用库
│   │   ├── src/
│   │   │   └── lib.rs   # 业务逻辑 + C导出层
│   │   └── Cargo.toml
│   ├── mathlib/         # 数学计算库
│   │   ├── src/
│   │   │   └── lib.rs   # 业务逻辑 + C导出层
│   │   └── Cargo.toml
│   └── stringlib/       # 字符串处理库
│       ├── src/
│       │   └── lib.rs   # 业务逻辑 + C导出层
│       └── Cargo.toml
├── apps/
│   ├── static-app/      # 静态链接版本（直接使用 Rust 库）
│   └── dynamic-app/     # 动态链接版本（libloading 加载 .dylib）
├── scripts/
│   └── build.sh         # 构建脚本
└── build/               # 构建输出
    ├── static/
    │   └── static-app
    └── dynamic/
        ├── dynamic-app
        └── lib/
            ├── libcommon.dylib
            ├── libmathlib.dylib
            └── libstringlib.dylib
```

## 快速开始

### 构建

```bash
./scripts/build.sh
```

### 运行

```bash
# 静态链接版本
./build/static/run.sh

# 动态链接版本
./build/dynamic/run.sh
```

两个版本功能完全相同，输出结果一致。

## 构建产物对比

详见 [构建测试总结.md](md/构建测试总结.md)

### 静态链接
- **static-app**: 530K
- **总计**: 530K

### 动态链接
- **dynamic-app**: 440K
- **libcommon.dylib**: 491K
- **libmathlib.dylib**: 509K
- **libstringlib.dylib**: 512K
- **总计**: 1.9M

⚠️ 动态链接版本比静态链接大 **72.85%**

### 与 Go 版本对比

| 语言 | 静态链接 | 动态链接 | 动态/静态比 |
|------|---------|---------|------------|
| Go   | 2.3M    | 7.4M    | 321%       |
| Rust | 530K    | 1.9M    | 368%       |

**Rust 优势**：
- 静态版本比 Go 小 **77%**
- 动态版本比 Go 小 **74%**
- 无运行时，无 GC，更激进的优化

## 技术要点

### Cargo Workspace
- 使用 `Cargo.toml` 的 `[workspace]` 统一管理 5 个模块
- 模块间依赖通过 `path` 指定
- 共享依赖版本，统一构建

### 动态库生成
- 使用 `crate-type = ["rlib", "cdylib"]` 同时生成静态和动态库
- 生成 `.dylib`（macOS）或 `.so`（Linux）
- C 导出层（`#[no_mangle]` + `extern "C"`）在同一个 `lib.rs` 中
- C 导出层只做类型转换，调用原始 Rust 代码（确保静态和动态版本逻辑一致）

### FFI 集成
- `dynamic-app` 使用 `libloading` 运行时加载动态库
- 使用 ID 映射管理 Rust 对象（避免跨 FFI 传递 Rust 指针）
- `rpath` 设置为 `@executable_path/lib`（相对路径）
- 使用 `CString`/`CStr` 处理字符串传递

## 验证动态链接

```bash
otool -L build/dynamic/dynamic-app
```

输出应显示对三个 `.dylib` 的依赖。

## 架构设计

### 代码复用
- **业务逻辑**: 在 `packages/*/src/lib.rs` 的 Rust 代码中实现
- **静态链接**: 直接使用 Rust 库
- **动态链接**: 同一个 lib.rs 文件包含 C 导出函数，通过 FFI 调用

关键：两种方式执行**完全相同**的代码。

### 模块依赖
```
common (基础库)
  ↓
mathlib, stringlib (依赖 common)
  ↓
static-app (直接使用 Rust 库)
dynamic-app (libloading 加载 .dylib)
```

## 环境要求

- Rust 1.70+
- Cargo
- macOS / Linux

---

**创建日期**: 2025年11月23日
