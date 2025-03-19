# GStreamer WebRTC Javascript API build

<https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/tree/0.13.5/net/webrtc/gstwebrtc-api>

```sh
# PWD <- Project Home 'programmable-matter-rpi/dev-projects/gst-webrtc/'

sudo apt install --no-install-recommends nodejs npm

git clone --depth 1 --branch 0.13.5 https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs.git
# rm -rf gst-plugins-rs/.git

cd gst-plugins-rs/net/webrtc/gstwebrtc-api

npm install

# npm WARN deprecated acorn-import-assertions@1.9.0: package has been renamed to acorn-import-attributes
# npm WARN deprecated inflight@1.0.6: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
# npm WARN deprecated glob@7.2.0: Glob versions prior to v9 are no longer supported
# npm WARN deprecated @humanwhocodes/config-array@0.11.14: Use @eslint/config-array instead
# npm WARN deprecated @humanwhocodes/object-schema@2.0.3: Use @eslint/object-schema instead
# npm WARN deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated rimraf@2.7.1: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated eslint@8.49.0: This version is no longer supported. Please see https://eslint.org/version-support for other options.
# 
# > gstwebrtc-api@2.0.0 postinstall
# > patch-package
# 
# patch-package 8.0.0
# Applying patches...
# webrtc-adapter@8.2.3 ✔
# 
# added 533 packages, and audited 534 packages in 41s
# 
# 4 moderate severity vulnerabilities
# 
# To address all issues, run:
#   npm audit fix --force
# 
# Run `npm audit` for details.

npm run make

# > gstwebrtc-api@2.0.0 make
# > npm run check && npm run build && npm run docs
#
#
# > gstwebrtc-api@2.0.0 check
# > eslint src
#
#
# > gstwebrtc-api@2.0.0 build
# > rimraf dist && webpack
#
# asset gstwebrtc-api-2.0.0.min.js 93.9 KiB [emitted] [minimized] (name: gstwebrtc-api) 1 related asset
# asset index.html 12.7 KiB [emitted]
# orphan modules 181 KiB [orphan] 20 modules
# runtime modules 937 bytes 4 modules
# cacheable modules 207 KiB
#   ./src/index.js + 20 modules 182 KiB [built] [code generated]
#   ./node_modules/sdp/sdp.js 24.7 KiB [built] [code generated]
# webpack 5.88.2 compiled successfully in 2420 ms
#
#> gstwebrtc-api@2.0.0 docs
#> rimraf docs && jsdoc src/*.js -d docs/ -p package.json -R README.md

ls -alh dist/

# total 436K
# drwxr-xr-x 2 cavani cavani 4.0K Mar 15 11:09 ./
# drwxr-xr-x 8 cavani cavani 4.0K Mar 15 11:09 ../
# -rw-r--r-- 1 cavani cavani  94K Mar 15 11:09 gstwebrtc-api-2.0.0.min.js
# -rw-r--r-- 1 cavani cavani 315K Mar 15 11:09 gstwebrtc-api-2.0.0.min.js.map
# -rw-r--r-- 1 cavani cavani  13K Mar 15 11:09 index.html

mkdir -p ../../../../web-app/
cp dist/* ../../../../web-app/
```

