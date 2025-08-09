#!/usr/bin/env sh

svdtools patch CH58Xxx.svd.patch
svd2rust -i CH58Xxx.svd.patched -o . -g -s -m --target riscv --settings CH58Xxx.yml --ident-formats-theme legacy
mv generic.rs src
mv mod.rs src/generated.rs
