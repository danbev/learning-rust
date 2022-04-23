
out/%: src/%.rs | out
	rustc -o $@ $<

out:
	@mkdir out

.PHONY: clean
clean:
	@${RM} -rf out
