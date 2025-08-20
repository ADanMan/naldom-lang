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

## Getting Started (v0.1.0-alpha)

The current version of Naldom is a working prototype that can compile natural language into an executable Python script.

### Step 1: Run the LLM Server

Naldom's prototype relies on a locally running `llama.cpp` server for inference. You need to build and run this server first.

```bash
# Clone the llama.cpp repository
git clone https://github.com/ggerganov/llama.cpp.git
cd llama.cpp

# Build with CMake and Metal support (for Apple Silicon)
mkdir build && cd build
cmake .. -DLLAMA_METAL=ON
cmake --build . --config Release
```

Once built, run the server from the `build` directory, pointing it to your model file. **Keep this terminal window open.**

```bash
# Make sure to replace the path to your model file
./bin/server -m /path/to/your/naldom-lang/llm/models/Qwen3-1.7B-Q8_0.gguf --host 127.0.0.1 --port 8080 -c 4096 -ngl 32
```

For more detailed instructions, see our [Development Setup Guide](docs/development-setup/llm-server-setup.md).

### Step 2: Compile and Run a Naldom Program

In a **new terminal window**, navigate to the `naldom-lang` project root.

First, create a file named `program.md` with the following content:
```markdown
:::naldom
Create an array of 10 random numbers.
Sort it in ascending order.
Print the result.
:::
```

Now, compile it using `naldomc` (via `cargo run`):
```bash
# This command generates a self-contained Python script
cargo run --package naldom-cli -- program.md -o output.py
```

Finally, execute the generated script:
```bash
python3 output.py
```

You should see the sorted array of random numbers printed to your console.

## Roadmap Highlights

✅ **Phase 1: Prototype** is complete! We have a working end-to-end pipeline that compiles natural language to Python.

Our ambitious future phases will include:
*   Developing our custom Low-Level Intermediate Representation (IR-LL).
*   Generating robust LLVM IR to support native binaries and WebAssembly.
*   Building a comprehensive and minimal cross-platform runtime.
*   Developing essential developer tools, including IDE plugins and a package manager.

You can find a more detailed roadmap in our [docs/roadmap.md](docs/roadmap.md).

## Contributing

We welcome contributions from everyone! Whether you're interested in language design, compiler development (Rust focus), LLM integration, or documentation, there's a place for you to make a significant impact.

Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to get started. We also encourage you to read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand our community standards.

Join our discussions on GitHub to share ideas, ask questions, and connect with other contributors!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for more details.