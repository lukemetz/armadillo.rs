CC ?= gcc
CXX ?= g++
CXXFLAGS = -fPIC -O3 -larmadillo

.PHONY: all
all: clean libcarmadillo.so
	cp ../libcarmadillo.so ../target/test/

libcarmadillo.so: f32_armadillo.cpp
	$(CXX) $(CXXFLAGS) -o ../libcarmadillo.so f32_armadillo.cpp --shared

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
