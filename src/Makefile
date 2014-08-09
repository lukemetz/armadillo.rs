CC ?= gcc
CXX ?= g++
CXXFLAGS = -fPIC -O3 -larmadillo
# quickly try and get the right python version
PYTHON ?= $(shell [ `which python2` ] && echo python2 || echo python)

.PHONY: all
all: clean libcarmadillo.so
	cp ../libcarmadillo.so ../target/test/

gen_armadillo.cpp:
	$(PYTHON) make_generated.py


libcarmadillo.so: gen_armadillo.cpp
	$(CXX) $(CXXFLAGS) -o ../libcarmadillo.so gen_armadillo.cpp --shared

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
	rm -rf gen_armadillo.cpp gen_armadillo.rs
