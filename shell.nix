with import <nixpkgs> { };

mkShell {

  name = "env";
  buildInputs = [
    python3Packages.python
    python3Packages.venvShellHook
    rustup rustc cargo wasm-pack openssl gccStdenv glibc pkg-config jbang emscripten nodejs cmake check protobuf protobufc pkg-config wasmtime wabt
  ];

  venvDir = "./.venv";
  postVenvCreation = ''
    unset SOURCE_DATE_EPOCH
    pip install urllib3
  '';

  RUSTC_VERSION = "nightly";
  postShellHook = ''
    # allow pip to install wheels
    unset SOURCE_DATE_EPOCH
    mkdir -p ~/.emscripten
    cp -rf ${emscripten}/share/emscripten/cache ~/.emscripten
    export EM_CACHE=~/.emscripten/cache
    export TMP=/tmp
    export TMPDIR=/tmp
    RUSTUP_HOME=~/.rustup
    rustup install $RUSTC_VERSION
    rustup default $RUSTC_VERSION
    export PATH=$PATH:~/.cargo/bin
    export PATH=$PATH:~/.rustup/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin
    echo ":openapi-demo:"
  '';

}