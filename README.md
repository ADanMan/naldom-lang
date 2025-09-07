# Naldom: Natural Language Domain Markdown

## Speak, and it is executed.

<!-- Badges -->
![Build Status](https://img.shields.io/github/actions/workflow/status/ADanMan/naldom-lang/rust-ci.yml?branch=main)
![License](https://img.shields.io/badge/license-MIT-blue)
![Contributors](https://img.shields.io/github/contributors/ADanMan/naldom-lang)

## About Naldom

Naldom (Natural Language Domain Markdown) is a next-generation programming language where source code is written in extended Markdown, and compilation occurs directly from Natural Language (NL) into executable code, bypassing traditional programming languages. Our mission is to bridge the gap between human thought and executable software, making programming more intuitive, transparent, and accessible.

## Core Principles & Goals

Naldom is built upon several core principles to make programming more intuitive and efficient:

*   **Simplicity:** Develop software using natural language descriptions, significantly reducing the learning curve and cognitive load associated with traditional programming syntax.
*   **Transparency:** Multi-level compilation, providing visibility and access to each stage of the process, from high-level intent to low-level machine instructions.
*   **Cross-Platform Compatibility:** Broad support across a wide range of operating systems and hardware, including Linux, macOS, Windows, iOS, Android, high-performance computing (CUDA, ROCm), WebAssembly, and embedded systems.
*   **Offline Operation:** The ability to run with minimal, local Large Language Models (LLM) for robust offline execution, ensuring privacy and availability.
*   **Contextual Optimization:** The language understands its execution environment and intelligently adapts or optimizes the generated code for optimal performance.

## How it Works: The Compilation Pipeline

Naldom transforms your natural language descriptions into executable code through an innovative multi-stage compilation pipeline:

1.  **User Input:** You write your ideas in a Markdown file (`.md` or `.nldm`).
2.  **NLD Parser (LLM):** A Natural Language Domain Parser, powered by a local LLM, interprets your input.
3.  **IntentGraph:** Your natural language is converted into a high-level, abstract representation of your program's intent.
4.  **Semantic Analysis & Optimization:** The IntentGraph undergoes semantic analysis and contextual optimization.
5.  **Intermediate Representations (IR-HL, IR-LL):** The high-level intent is progressively refined into lower-level representations.
6.  **Code Generation:** Target-specific code is generated for various platforms (CPU, GPU, WebAssembly, etc.).
7.  **Runtime Packaging:** The final executable binary, WASM module, or object code is packaged for deployment.

This sophisticated process eliminates the need for traditional, syntax-heavy programming, allowing you to focus on *what* you want to achieve, not *how* to write it in specific code.

## Getting Started (v0.2.0-alpha)

The current version of Naldom can compile natural language into a native executable or a WebAssembly module.

### Prerequisites

To build and run Naldom, you will need the following installed on your system:

1.  **Rust Toolchain:** Install via [rustup.rs](https://rustup.rs/).
2.  **LLVM 17:** We recommend installing via your system's package manager (e.g., `brew install llvm@17` on macOS).
3.  **Clang:** Usually installed as part of the LLVM package.
4.  **llama.cpp:** Naldom requires a locally running `llama.cpp` server. Please follow the **[LLM Server Setup Guide](docs/development-setup/llm-server-setup.md)**.

### Step 1: Run the LLM Server

Before using the compiler, you must have a `llama.cpp` server running. In a separate terminal, run a command similar to this (adjust the model path as needed):

```bash
# From your llama.cpp/build directory
./bin/server -m /path/to/your/model.gguf --host 127.0.0.1 --port 8080
```

For more detailed instructions, see our [Development Setup Guide](docs/development-setup/llm-server-setup.md).

### Step 2: Compile and Run a Naldom Program

In a new terminal, navigate to the `naldom-lang` project root.

First, create a file named `program.md` with the following content:
```markdown
:::naldom
Create an array of 10 random numbers.
Sort it in ascending order.
Print the result.
:::
```

Now, compile and run it with a single command:
```bash
# Set the LLVM_PREFIX if your LLVM installation is not in the system PATH
export LLVM_PREFIX=$(brew --prefix llvm@17) # Example for macOS Homebrew

# Compile and run the native executable with optimizations
cargo run --package naldom-cli -- program.md --run -O2
```

You should see the sorted array of random numbers printed to your console.

To compile to **WebAssembly**, use the `--target` flag:
```bash
cargo run --package naldom-cli -- program.md --target wasm -o program.wasm
```

## Roadmap Highlights

✅ **Phase 1: Prototype**
✅ **Phase 2: Core Compiler & Multi-Target**

We are now entering **Phase 3: Full Compiler & Ecosystem**, which will focus on building a rich ecosystem around the compiler, including:
*   A Foreign Function Interface (FFI).
*   A robust standard library.
*   IDE support via a Language Server Protocol (LSP) server.
*   A package manager (`naldom-pkg`).

You can find a more detailed roadmap in our [docs/roadmap.md](docs/roadmap.md).

## Contributing

We welcome contributions from everyone! Whether you're interested in language design, compiler development (Rust focus), LLM integration, or documentation, there's a place for you to make a significant impact.

Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to get started. We also encourage you to read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand our community standards.

Join our discussions on GitHub to share ideas, ask questions, and connect with other contributors!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.