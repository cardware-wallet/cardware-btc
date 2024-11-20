his is dependant on both RUST and having a C Linker.

Rust can be installed by following these instructions:

	https://www.rust-lang.org/tools/install

Or on Mac OS X using this curl command:

	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

The easiest way to handle getting a C-Linker on Mac OS X is to use homebrew:

Install Homebrew using this command: 

	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

Once homebrew is installed you can install LLVM using the command:

	brew install llvm

Ensure the C-Linker is available to the Rust Compiler by using an export:

	export TARGET_CC=/opt/homebrew/Cellar/llvm/17.0.6_1/bin/clang-17

It is important to note that the path to llvm can change depending on version number and the users OS.

You will also need webpack on the NPM side, install using this command:

	npm i webpack

When building one must first use the command:

	wasm-pack build

followed by:

	npm run serve


REMEBER:
	
	Remember to comment out tokio when building for wasm-pack.

EDGE CASE ERROR:

If you see an error that looks like this:

  Internal error occurred: Command "/opt/homebrew/Cellar/llvm/17.0.2/bin/clang-17" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-Wall" "-Wextra" "-o" "/Users/dom/Documents/Rust/scl_wallet/target/wasm32-unknown-unknown/release/build/rust-crypto-wasm-71b19d49c9aa1fe2/out/src/util_helpers.o" "-c" "src/util_helpers.c" with args "clang-17" did not execute successfully (status code exit status: 1).

This is caused by a rust compilation order of operations problem

The solution is to:

	1) Exit your terminal window and quit terminal

	2) Navigate to your working directory and run "wasm-pack build" this will fail with a C_Linker error

	3) Export your C_Linker using: "export TARGET_CC=/opt/homebrew/Cellar/llvm/18.1.8/bin/clang-18"

	4) Re-run "wasm-pack build" this time it will work (black magic!)
