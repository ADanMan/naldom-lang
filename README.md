# Naldom: Natural Language Domain Markdown

## Speak, and it is executed.

<!-- Placeholder for future badges -->
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Contributors](https://img.shields.io/github/contributors/ADanMan/naldom-lang)

## About Naldom

Naldom (Natural Language Domain Markdown) is a next-generation programming language where source code is written in extended Markdown, and compilation occurs directly from Natural Language (NL) into executable code, bypassing traditional programming languages. Our mission is to bridge the gap between human thought and executable software, making programming more intuitive, transparent, and accessible.

## Core Principles & Goals

Naldom is built upon several core principles to make programming more intuitive and efficient:

*   **Simplicity:** Develop software using natural language descriptions, significantly reducing the learning curve and cognitive load associated with traditional programming syntax.
*   **Transparency:** Multi-level compilation, providing visibility and access to each stage of the process, from high-level intent to low-level machine instructions.
*   **Cross-Platform Compatibility:** Broad support across a wide range of operating systems and hardware, including Linux, macOS, Windows, iOS, Android, high-performance computing (CUDA, ROCm), WebAssembly, and embedded systems.
*   **Offline Operation:** The ability to run with minimal, local Large Language Models (LLM) (e.g., 1.5-2B parameters like Qwen-1.5B distilled or Phi-2) for robust offline execution, ensuring privacy and availability without constant internet access.
*   **Contextual Optimization:** The language understands its execution environment (hardware, available resources, data characteristics) and intelligently adapts or optimizes the generated code for optimal performance and resource utilization.

## How it Works: The Compilation Pipeline

Naldom transforms your natural language descriptions into executable code through an innovative multi-stage compilation pipeline:

1.  **User Input:** You write your ideas and program logic in a Markdown file with Naldom extensions (`.md` or `.nldm`).
2.  **NLD Parser (LLM):** A Natural Language Domain Parser, powered by a local Large Language Model (LLM), interprets your input, understanding the semantic intent behind your natural language instructions.
3.  **IntentGraph:** Your natural language is converted into a high-level, abstract representation of your program's intent, capturing the core logic independent of specific implementation details.
4.  **Semantic Analysis & Optimization:** The IntentGraph undergoes rigorous semantic analysis, including type inference and dependency checking. It is then subjected to contextual optimization, tailoring the program for its intended execution environment.
5.  **Intermediate Representations (IR-HL, IR-LL):** The high-level intent is progressively refined into a High-Level Intermediate Representation (IR-HL) and then a Low-Level Intermediate Representation (IR-LL), which is closer to machine-level instructions and often leverages LLVM IR.
6.  **Code Generation:** Target-specific code is generated for various supported platforms and architectures (CPU, GPU, WebAssembly, embedded).
7.  **Runtime Packaging:** The final executable binary, WASM module, or object code is packaged for deployment and execution.

This sophisticated process eliminates the need for traditional, syntax-heavy programming, allowing you to focus on *what* you want to achieve, not *how* to write it in specific code.

## Running the Project (Prototype Phase)

During the prototype phase, Naldom relies on a locally running `llama.cpp` server for LLM inference. This approach was chosen to ensure stability and bypass complex build issues.

**Step 1: Run the `llama.cpp` Server**

You need to clone and build `llama.cpp` separately.

```bash
# Clone the repository
git clone https://github.com/ggerganov/llama.cpp.git
cd llama.cpp

# Build with CMake and Metal support (for Apple Silicon)
mkdir build && cd build
cmake .. -DLLAMA_METAL=ON
cmake --build . --config Release
```

Once built, run the server from the `build` directory, pointing it to your model file. **Keep this terminal window open.**

```bash
./bin/server -m /path/to/your/naldom-lang/llm/models/Qwen3-1.7B-Q8_0.gguf --host 127.0.0.1 --port 8080 -c 4096 -ngl 32
```

**Step 2: Run Naldom**

In a **new terminal window**, navigate to the `naldom-lang` project root and run the compiler:

```bash
cargo run --package naldom-cli -- program.md
```

For more detailed instructions, see our [Development Setup Guide](docs/development-setup/llm-server-setup.md).

## Quick Start

Naldom is currently in a working prototype phase. You can compile natural language into an executable Python script.

Imagine a file named `my_program.md`:

```markdown
:::naldom
Create an array of 10 random numbers.
Sort it in ascending order.
Print the result.
:::
```

To compile this, you first need to run the `llama.cpp` server (see the "Running the Project" section below for details). Then, execute the following command:

```bash
# This will generate a self-contained Python script
naldomc my_program.md -o my_program.py
```

And to run it:

```bash
python3 my_program.py
# Expected output:
# Runtime: Creating an array of 10 random numbers...
# Runtime: Sorting the array in ascending order...
#
# --- Naldom Output ---
#
# ---------------------
```

## Roadmap Highlights

We are currently in **Phase 1: Prototype** development, focusing on building the core Natural Language Domain Parser (NLD Parser) and generating initial executable code (e.g., Python for rapid prototyping and validation).

Our ambitious future phases will include:
*   Developing our custom Low-Level Intermediate Representation (IR-LL).
*   Generating robust LLVM IR and supporting a wide array of multi-targets like WebAssembly, native binaries for various CPU/GPU architectures, and embedded systems.
*   Building a comprehensive and minimal cross-platform runtime for efficient execution.
*   Developing essential developer tools, including IDE plugins (e.g., for VS Code), an interactive shell (`naldom-repl`), an analyzer for visualizing internal representations, and a package manager (`naldom-pkg`).
*   Implementing advanced security features and reverse traceability to enhance reliability and debugging.

You can find a more detailed roadmap in our [docs/roadmap.md](docs/roadmap.md) (this file will be created as part of the next steps).

## Contributing

We welcome contributions from everyone! Whether you're interested in language design, compiler development (Rust focus), LLM integration, runtime optimization, developer tooling, or documentation, there's a place for you to make a significant impact.

Before you start, please read our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to get started, report bugs, suggest features, or submit pull requests. We also encourage you to read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand our community standards.

Join our discussions on GitHub to share ideas, ask questions, and connect with other contributors!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for more details.
