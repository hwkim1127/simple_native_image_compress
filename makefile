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

# # From https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
# install_cargokit:
# 	git subtree add --prefix cargokit https://github.com/irondash/cargokit.git main --squash

# # Unlike git modules, subtrees actually live in your repository and thus work with pub without any issues.
# update_cargokit:
# 	git subtree pull --prefix cargokit https://github.com/irondash/cargokit.git main --squash

generate_bindings:
	flutter_rust_bridge_codegen generate

check_plugin:
	flutter pub publish --dry-run

publish:
	flutter pub publish