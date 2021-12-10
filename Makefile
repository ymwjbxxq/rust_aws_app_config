FUNCTIONS := handler
STACK_NAME := rust-app-config
ARCH := x86_64-unknown-linux-musl
# ARCH := aarch64-unknown-linux-gnu # AppConfig is still not supporting it

build:
	rm -rf ./build
	rm -rf ./target
	cross build --features vendored-openssl --features rustls --release --target $(ARCH)
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME} --template-file template.yml

delete:
	sam delete --profile test --stack-name ${STACK_NAME}