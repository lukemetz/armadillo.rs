CC = gcc
CXX = g++
CXXFLAGS = -fPIC -O3 -larmadillo -static -lstdc++ -c

.PHONY: all
all: clean libcarmadillo.a
	mv libcarmadillo.a ../

libcarmadillo.a: f32_armadillo.o
	ar rcs libcarmadillo.a f32_armadillo.o

f32_armadillo.o: f32_armadillo.cpp
	$(CXX) $(CXXFLAGS) f32_armadillo.cpp -fPIC

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
