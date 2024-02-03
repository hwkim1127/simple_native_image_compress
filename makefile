setup_apple:
	cargo install cargo-xcode
	cargo install cargo-lipo

setup_android: 
	cargo install cargo-ndk
	rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android

setup_ios:
	rustup target add \
	aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

install_cargokit:
	git subtree add --prefix cargokit https://github.com/irondash/cargokit.git main --squash

generate_bindings:
	flutter_rust_bridge_codegen \
	--rust-input rust/src/api.rs \
	--dart-output lib/src/bridge_generated.dart \
	--dart-decl-output lib/src/bridge_definitions.dart \
	--c-output ios/Classes/bridge_generated.h \
	--extra-c-output-path macos/Classes/ \
