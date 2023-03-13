#SHELL := /bin/bash
NAME = libur_registry_ffi
SO = $(NAME).so
LIB = $(NAME).a

ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
ARCHS_IOS = aarch64-apple-ios

# ANDROID_DEST = ~/Documents/Projects/KeystoneSDKAndroid/library/src/main/jniLibs
IOS_DEST = ../ios

# all: install-android ios
all: install-android

android: $(ARCHS_ANDROID)
ios: $(LIB)

clean:
	rm -R $(ANDROID_DEST)
	rm $(IOS_DEST)/LIB


# Android
aarch64-linux-android:
	cargo ndk -t arm64-v8a build --release

armv7-linux-androideabi:
	cargo ndk -t armeabi-v7a build --release

i686-linux-android:
	cargo ndk -t x86 build --release

x86_64-linux-android:
	cargo ndk -t x86_64 build --release

install-android: android
	mkdir -p $(ANDROID_DEST)
	mkdir -p $(ANDROID_DEST)/x86
	mkdir -p $(ANDROID_DEST)/x86_64
	mkdir -p $(ANDROID_DEST)/arm64-v8a
	mkdir -p $(ANDROID_DEST)/armeabi-v7a

	cp ../../target/i686-linux-android/release/$(SO) ${ANDROID_DEST}/x86/$(SO)
	cp ../../target/x86_64-linux-android/release/$(SO) ${ANDROID_DEST}/x86_64/$(SO)
	cp ../../target/aarch64-linux-android/release/$(SO) ${ANDROID_DEST}/arm64-v8a/$(SO)
	cp ../../target/armv7-linux-androideabi/release/$(SO) ${ANDROID_DEST}/armeabi-v7a/$(SO)


# iOS
# .PHONY: $(ARCHS_IOS)
# $(ARCHS_IOS): %:
# 	cargo build --target $@ --release

# $(LIB): $(ARCHS_IOS)
# 	mkdir -p $(IOS_DEST)
# 	lipo -create -output $(IOS_DEST)/$@ $(foreach arch,$(ARCHS_IOS),$(wildcard target/$(arch)/release/$(LIB)))