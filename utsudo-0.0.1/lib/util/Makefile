#!/bin/bash

all:
	make clean
	cargo build --release
	mkdir .libs 
	cp target/release/libutsudo_util.so  .libs/libutsudo_util.so.0.0.0
	cd .libs && ln -s libutsudo_util.so.0.0.0 libutsudo_util.so.0
	cd .libs && ln -s libutsudo_util.so.0.0.0 libutsudo_util.so
	cd .libs && ln -s ../libutsudo_util.la libutsudo_util.la
	cd .libs && cp ../libutsudo_util.lai ./
	cd .libs && patchelf --set-soname libutsudo_util.so.0 libutsudo_util.so.0.0.0
	make debug_fatal_lbuf.o


shlib_exp = 
shlib_enable = yes
LT_LDFLAGS =   -export-symbols 
INSTALL_BACKUP =
LIBTOOL = $(SHELL) $(top_builddir)/libtool


#### Start of system configuration section. ####

srcdir = .
devdir = $(srcdir)
top_builddir = ../..
top_srcdir = ../..
incdir = $(top_srcdir)/include
cross_compiling = no

# Where to install things...
prefix = /usr
exec_prefix = /usr
bindir = /usr/bin
sbindir = /usr/sbin
sysconfdir = /etc
libexecdir = /usr/libexec
datarootdir = $(prefix)/share
localstatedir = /var

# File extension, mode and map file to use for shared libraries/objects
shlib_enable = yes
shlib_mode = 0644
shlib_exp = 
shlib_map = 
shlib_opt = 

# Compiler & tools to use
CC = gcc
LIBTOOL = $(SHELL) $(top_builddir)/libtool
SED = /usr/bin/sed
AWK = gawk

# Our install program supports extra flags...
INSTALL = $(SHELL) $(top_srcdir)/install-sh -c
INSTALL_OWNER = -o $(install_uid) -g $(install_gid)
INSTALL_BACKUP = 

# C preprocessor defines
CPPDEFS = 

# C preprocessor flags
CPPFLAGS = -I$(incdir) -I$(top_builddir) -I$(srcdir) -I$(top_srcdir) $(CPPDEFS) -DZLIB_CONST -D_FORTIFY_SOURCE=2

# Usually -O and/or -g
CFLAGS = -g -O2

# Flags to pass to the link stage
LDFLAGS = 
LT_LDFLAGS =   -export-symbols $(shlib_exp)

# Flags to pass to libtool
LTFLAGS = --tag=disable-static

# Address sanitizer flags
ASAN_CFLAGS = 
ASAN_LDFLAGS = 

# PIE flags
PIE_CFLAGS = 
PIE_LDFLAGS = 

# Stack smashing protection flags
SSP_CFLAGS = 
SSP_LDFLAGS = 

# Libtool style shared library version
SHLIB_VERSION = 0:0:0

# cppcheck options, usually set in the top-level Makefile
CPPCHECK_OPTS = -q --force --enable=warning,performance,portability --suppress=constStatement --error-exitcode=1 --inline-suppr -Dva_copy=va_copy -U__cplusplus -UQUAD_MAX -UQUAD_MIN -UUQUAD_MAX -U_POSIX_HOST_NAME_MAX -U_POSIX_PATH_MAX -U__NBBY -DNSIG=64

# splint options, usually set in the top-level Makefile
SPLINT_OPTS = -D__restrict= -checks

# PVS-studio options
PVS_CFG = $(top_srcdir)/PVS-Studio.cfg
PVS_IGNORE = 'V707,V011,V002,V536'
PVS_LOG_OPTS = -a 'GA:1,2' -e -t errorfile -d $(PVS_IGNORE)

# Regression tests
TEST_PROGS = conf_test hltq_test parseln_test progname_test strsplit_test \
	     strtobool_test strtoid_test strtomode_test strtonum_test \
	     parse_gids_test getgrouplist_test strsig_test
TEST_LIBS = 
TEST_LDFLAGS = 

# User and group ids the installed files should be "owned" by
install_uid = 0
install_gid = 0

# Set to non-empty for development mode
DEVEL = 

#### End of system configuration section. ####

SHELL = /bin/sh

debug_fatal_lbuf.o: $(srcdir)/debug_fatal_lbuf.c $(incdir)/compat/stdbool.h \
             $(incdir)/sudo_compat.h $(incdir)/sudo_conf.h \
             $(incdir)/sudo_debug.h $(incdir)/sudo_dso.h \
             $(incdir)/sudo_fatal.h $(incdir)/sudo_gettext.h \
             $(incdir)/sudo_plugin.h $(incdir)/sudo_queue.h \
             $(top_builddir)/config.h \
             $(top_builddir)/pathnames.h
	$(CC) -c $(CPPFLAGS) $(CFLAGS) $(ASAN_CFLAGS) $(PIE_CFLAGS) $(SSP_CFLAGS) $(srcdir)/debug_fatal_lbuf.c

debug_fatal_lbuf.i: $(srcdir)/debug_fatal_lbuf.c $(incdir)/compat/stdbool.h \
             $(incdir)/sudo_compat.h $(incdir)/sudo_conf.h \
             $(incdir)/sudo_debug.h $(incdir)/sudo_dso.h \
             $(incdir)/sudo_fatal.h $(incdir)/sudo_gettext.h \
             $(incdir)/sudo_plugin.h $(incdir)/sudo_queue.h \
             $(top_builddir)/config.h \
             $(top_builddir)/pathnames.h
	$(CC) -E -o $@ $(CPPFLAGS) $<

debug_fatal_lbuf.plog: debug_fatal_lbuf.i
	rm -f $@; pvs-studio --cfg $(PVS_CFG) --sourcetree-root $(top_srcdir) --skip-cl-exe yes --source-file $(srcdir)/debug_fatal_lbuf.c --i-file $< --output-file $@


pre-install:

install: install-dirs
	case "$(LT_LDFLAGS)" in \
    *-no-install*) ;; \
    *)  if [ X"$(shlib_enable)" = X"yes" ]; then \
        INSTALL_BACKUP='$(INSTALL_BACKUP)' $(LIBTOOL) $(LTFLAGS) --quiet --mode=install $(INSTALL) $(INSTALL_OWNER) libutsudo_util.la $(DESTDIR)$(libexecdir)/utsudo; \
        fi;; \
    esac

install-dirs:
	$(SHELL) $(top_srcdir)/mkinstalldirs $(DESTDIR)$(libexecdir)/utsudo

install-binaries:

install-includes:

install-doc:

install-plugin:

uninstall:
	$(LIBTOOL) $(LTFLAGS) --mode=uninstall rm -f $(DESTDIR)$(libexecdir)/utsudo/libutsudo_util.la
	-test -z "$(INSTALL_BACKUP)" || \
		rf -f $(DESTDIR)$(libexecdir)/utsudo/libutsudo_util.*~
check:


clean:
	rm .libs -rf
	rm target -rf
	rm *.o -rf

.PHONY: clean
