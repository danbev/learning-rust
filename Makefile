#RUSTC =/home/danielbevenius/work/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
RUSTC = rustc
RUSTC_FLAGS ="-Copt-level=0" "--edition=2021"

out/%: src/%.rs | out
	${RUSTC} ${RUSTC_FLAGS} -o $@ -g $<

out:
	mkdir -p out

expand: out/simple_macro
	${RUSTC} ${RUSTC_FLAGS} -Zunpretty=expanded src/simple_macro.rs

ast-tree: out/simple_macro
	${RUSTC} ${RUSTC_FLAGS} -Zunpretty=ast-tree src/simple_macro.rs

ast: out/simple_macro
	${RUSTC} ${RUSTC_FLAGS} -Zunpretty=ast src/simple_macro.rs

hir: out/simple_macro
	${RUSTC} ${RUSTC_FLAGS} -Zunpretty=hir src/simple_macro.rs

mir: out/simple_macro
	${RUSTC} ${RUSTC_FLAGS} -Zunpretty=mir src/simple_macro.rs

out/async_core: src/async_core.rs
	${RUSTC} ${RUSTC_FLAGS} -o $@ -g $< \
		-L dependency=/home/danielbevenius/work/rust/learning-rust/async/target/debug/deps \
		--extern futures=/home/danielbevenius/work/rust/learning-rust/async/target/debug/deps/libfutures-db5560b305d383df.rlib

out/unsafecell_2.ll: src/unsafecell_2.rs
	${RUSTC} ${RUSTC_FLAGS} --emit=llvm-ir -o $@ $<

gdb_unsafecell: out/unsafecell
	gdb --args ${RUSTC} ${RUSTC_FLAGS} -Zmutable-noalias -g -o $<  src/unsafecell.rs

gdb_unsafecell_2: out/unsafecell_2
	gdb --args ${RUSTC} ${RUSTC_FLAGS} -Zmutable-noalias -g -o $<  src/unsafecell_2.rs

out/mono-filtered.ll: src/mono.rs
	${RUSTC} ${RUSTC_FLAGS} --emit=llvm-ir -o out/mono.ll $<
	@${RM} $@
	rustfilt --input out/mono.ll --output $@

.PHONY: clean
clean:
	@${RM} -rf out
