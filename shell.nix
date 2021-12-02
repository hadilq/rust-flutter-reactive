let
  # Mozilla Overlay
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
    config.android_sdk.accept_license = true;
  };

  frameworks = nixpkgs.darwin.apple_sdk.frameworks;
  rust =
    (nixpkgs.rustChannelOf {
      rustToolchain = ./rust-toolchain;
    }).rust.override {
      targets = [
        "aarch64-linux-android"
        "armv7-linux-androideabi"
        "x86_64-linux-android"
        "i686-linux-android"
      ] ++ (
        nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
          "aarch64-apple-ios"
          "x86_64-apple-ios"
        ]
      );
      extensions = [
        "clippy-preview"
        "rust-src"
      ];
    };

  androidComposition = nixpkgs.androidenv.composeAndroidPackages {
    platformVersions = [ "28" ];
    includeNDK = true;
  };

in
  with nixpkgs;

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
      rust
      androidComposition.androidsdk
    ];

    nativeBuildInputs = [
      clang
      llvm
      flatbuffers
      libiconv
      cargo-make
    ] ++ (
      lib.optionals stdenv.isDarwin [
        frameworks.Security
        frameworks.CoreServices
        frameworks.CoreFoundation
        frameworks.Foundation
      ]
    );

    # ENV Variables
    RUST_BACKTRACE = 1;
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
    ANDROID_HOME = androidComposition.androidsdk;
    ANDROID_NDK_HOME = "${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle";
    RUST_HOME = "${rust}";
    RUST_SRC_HOME = "${rust}/lib/rustlib/src/rust";

    # Post Shell Hook
    shellHook = ''
      echo "Using ${rust.name}"

      export PS1="\e[0;32m [$name] \[$txtgrn\]\u@\h\[$txtwht\]:\[$bldpur\]\w \[$txtcyn\]\$git_branch\[$txtred\]\$git_dirty \[$bldylw\]\$aws_env\[$txtrst\]\\e[m \n$ "
    '' + (
      if !pkgs.stdenv.isDarwin then
        ""
      else ''
        # Cargo wasn't able to find CF during a `cargo test` run on Darwin.
        export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
      ''
    );
  }

