# Setting up the llama.cpp Server

The current prototype of Naldom uses a client-server architecture for LLM inference. The `naldomc` compiler acts as a client, sending requests to a locally running `llama.cpp` server. This guide explains how to set up this server.

## 1. Prerequisites

- A C++ compiler toolchain (Xcode Command Line Tools on macOS, Build Essentials on Linux).
- `cmake` (can be installed via `brew install cmake` on macOS).

## 2. Build `llama.cpp`

First, clone the `llama.cpp` repository to a location outside of the `naldom-lang` project.

```bash
git clone https://github.com/ggerganov/llama.cpp.git
cd llama.cpp
```

Next, build the project using `CMake`. The following command is for Apple Silicon Macs with Metal GPU support.

```bash
mkdir build
cd build
cmake .. -DLLAMA_METAL=ON
cmake --build . --config Release
```

For other platforms (Linux with CUDA, Windows, CPU-only), please refer to the official `llama.cpp` build documentation.

## 3. Run the Server

Once the build is complete, you can start the server. The `server` executable will be located in the `build/bin` directory.

You must provide the path to the model file you want to use.

```bash
./bin/server -m /path/to/your/naldom-lang/llm/models/Qwen3-1.7B-Q8_0.gguf --host 127.0.0.1 --port 8080 -c 4096 -ngl 32
```

- `-m`: Path to your GGUF model file.
- `--host` and `--port`: The address and port for the server. Naldom expects `127.0.0.1:8080` by default.
- `-c`: Context size. `4096` is a safe value.
- `-ngl`: Number of GPU layers to offload. `32` is a good starting point for Apple Silicon.

**Keep this terminal window running** while you are working on Naldom.