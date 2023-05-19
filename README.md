# 我的项目

这是一个使用 Rust（编译为 WASM）和 Node.js 的网络爬虫项目。

## 项目结构
```sh
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── other_rust_files.rs
├── wasm/
│   ├── wasm_pack_script.sh
│   └── pkg/
│       ├── my_project_bg.wasm
│       ├── my_project.d.ts
│       └── my_project.js
├── parse/
│   ├── package.json
│   ├── server.js
│   └── other_js_files.js
└── README.md
```

src/ 目录包含了 Rust 源代码。它由 Cargo.toml 文件管理，该文件描述了 Rust 项目的配置和依赖。

wasm/ 目录是 Rust 代码被编译为 WASM 之后的输出位置。wasm_pack_script.sh 是一个脚本，用于运行 wasm-pack 命令，将 Rust 代码编译为 WASM，并输出到 pkg/ 子目录。

parse/ 目录包含 Node.js 代码和资源。它由 package.json 文件管理，该文件描述了 Node.js 项目的配置和依赖。server.js 可以是主要 Node.js 脚本，它加载和运行 WASM 模块，然后处理 HTML。


## 安装

### Rust 和 wasm-pack

你需要先安装 Rust 和 wasm-pack。安装指南可在 [Rust 官方网站](https://www.rust-lang.org/tools/install) 和 [wasm-pack 官方网站](https://rustwasm.github.io/wasm-pack/installer/) 找到。

> 安装 wasm-pack:
 `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`


### Node.js 和 npm

你还需要安装 Node.js 和 npm。安装指南可在 [Node.js 官方网站](https://nodejs.org/) 找到。

### 项目依赖

你可以使用以下命令安装项目的依赖：

```bash
# 在 Rust 中安装依赖
cargo build

# 在 Node.js 中安装依赖
cd node
npm install

# 运行
你需要编译 Rust 代码为 WASM：
./wasm/wasm_pack_script.sh


