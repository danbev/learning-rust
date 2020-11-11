.PHONY: clean

libcfunctions.so: cfunctions.o
	${CC} --shared -o $@ $<

cfunctions.o: cfunctions.h cfunctions.c
	${CC} -fPIC -o $@ -c cfunctions.c -I.

something: something.rs libcfunctions.so
	rustc -L. -lcfunctions something.rs -C link-arg='-Wl,-rpath,${PWD}'
	@# The command below can be used if the library to link with is
	@# specified in something.rs.
	@#rustc -L. something.rs -C link-arg='-Wl,-rpath,${PWD}'
clean: 
	${RM} libcfunctions.so cfunctions.o something
	
