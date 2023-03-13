.PHONY: all clean

OBJS:=\
	16_traits_rs \
	16_traits_cpp \
	16-2_dyn_rs \
	16-2_dyn_cpp

CXXFLAGS=-std=c++17

all: ${OBJS}

clean:
	rm -f ${OBJS}

%_rs: %.rs
	rustc ${RSFLAGS} $< -o $@

%_cpp: %.cpp
	g++ ${CXXFLAGS} $< -o $@
