
out/%: src/%.rs | out
	rustc -o $@ -g $<

out:
	@mkdir out

.PHONY: clean
clean:
	@${RM} -rf out
