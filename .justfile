build-release-native:
    @cargo build --profile release-native --no-default-features
run-release-native:
    @cargo run --profile release-native --no-default-features