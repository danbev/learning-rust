
out/%: src/%.rs | out
	rustc --edition 2021 -o $@ -g $<

out:
	@mkdir out

.PHONY: clean
clean:
	@${RM} -rf out
