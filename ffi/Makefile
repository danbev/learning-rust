.PHONY: clean

libcfunctions.so: cfunctions.o
	${CC} --shared -o $@ $<

cfunctions.o: cfunctions.h cfunctions.c
	${CC} -fPIC -o $@ -c cfunctions.c -I.

something: something.rs libcfunctions.so
	rustc -L. something.rs -C link-arg='-Wl,-rpath,${PWD}'
clean: 
	${RM} libcfunctions.so cfunctions.o something
	
