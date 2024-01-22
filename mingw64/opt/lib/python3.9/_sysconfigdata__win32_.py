import sys
# system configuration generated and used by the sysconfig module
build_time_vars = {'ABIFLAGS': '',
 'AC_APPLE_UNIVERSAL_BUILD': 0,
 'AIX_BUILDDATE': 0,
 'AIX_GENUINE_CPLUSPLUS': 0,
 'ALT_SOABI': 0,
 'ANDROID_API_LEVEL': 0,
 'AR': 'ar',
 'ARFLAGS': 'rcs',
 'BASECFLAGS': '-Wno-unused-result -Wsign-compare',
 'BASECPPFLAGS': '-IObjects -IInclude -IPython',
 'BASEMODLIBS': '',
 'BINDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/bin',
 'BINLIBDEST': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9',
 'BLDLIBRARY': '-L. -lpython3.9',
 'BLDSHARED': 'x86_64-w64-mingw32-gcc -shared -Wl,--enable-auto-image-base '
              '-pipe -fno-ident '
              '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
              '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
              '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
              '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
              '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'BUILDEXE': '.exe',
 'BUILDPYTHON': 'python.exe',
 'BUILDPYTHONW': 'pythonw.exe',
 'BUILDVENVLAUNCHER': 'venvlauncher.exe',
 'BUILDVENVWLAUNCHER': 'venvwlauncher.exe',
 'BUILD_GNU_TYPE': 'x86_64-w64-mingw32',
 'BYTESTR_DEPS': '\\',
 'CC': 'x86_64-w64-mingw32-gcc',
 'CCSHARED': '',
 'CFLAGS': '-Wno-unused-result -Wsign-compare -DNDEBUG -g -fwrapv -O3 -Wall '
           '-O2 -pipe -fno-ident '
           '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
           '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
           '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
           '-D__USE_MINGW_ANSI_STDIO=1',
 'CFLAGSFORSHARED': '',
 'CFLAGS_ALIASING': '',
 'CONFIGFILES': 'configure configure.ac acconfig.h pyconfig.h.in '
                'Makefile.pre.in',
 'CONFIGURE_CFLAGS': '-O2 -pipe -fno-ident '
                     '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                     '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                     '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                     '-D__USE_MINGW_ANSI_STDIO=1',
 'CONFIGURE_CFLAGS_NODIST': '-std=c99 -Wextra -Wno-unused-result '
                            '-Wno-unused-parameter '
                            '-Wno-missing-field-initializers '
                            '-Wstrict-prototypes '
                            '-Werror=implicit-function-declaration '
                            '-fvisibility=hidden -D_WIN32_WINNT=0x0601 '
                            '-DMS_DLL_ID=\'"3.9"\'',
 'CONFIGURE_CPPFLAGS': '-I../../../src/Python-3.9.10/PC  '
                       '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                       '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                       '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                       '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                       '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                       '-IC:/buildroot/prerequisites/x86_64-zlib-static/include '
                       '-I.',
 'CONFIGURE_LDFLAGS': '-pipe -fno-ident '
                      '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
                      '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
                      '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
                      '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
                      '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'CONFIGURE_LDFLAGS_NODIST': '',
 'CONFIG_ARGS': "'--host=x86_64-w64-mingw32' '--build=x86_64-w64-mingw32' "
                "'--prefix=C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt' "
                "'--enable-shared' '--with-system-expat' '--with-system-ffi' "
                "'--enable-loadable-sqlite-extensions' '--without-ensurepip' "
                "'--without-c-locale-coercion' '--enable-optimizations' "
                "'LIBFFI_INCLUDEDIR=C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/libffi-3.2.1/include' "
                "'CFLAGS=-O2 -pipe -fno-ident "
                '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                "-D__USE_MINGW_ANSI_STDIO=1' 'CPPFLAGS= "
                '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                "-IC:/buildroot/prerequisites/x86_64-zlib-static/include' "
                "'LDFLAGS=-pipe -fno-ident "
                '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
                '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
                '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
                '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
                "-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib' "
                "'build_alias=x86_64-w64-mingw32' "
                "'host_alias=x86_64-w64-mingw32'",
 'CONFINCLUDEDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include',
 'CONFINCLUDEPY': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/python3.9',
 'COREPYTHONPATH': '',
 'COVERAGE_INFO': '/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/build/Python-3.9.10/coverage.info',
 'COVERAGE_REPORT': '/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/build/Python-3.9.10/lcov-report',
 'COVERAGE_REPORT_OPTIONS': '--no-branch-coverage --title "CPython lcov '
                            'report"',
 'CPPFLAGS': '-IObjects -IInclude -IPython -I. '
             '-I../../../src/Python-3.9.10/Include '
             '-I../../../src/Python-3.9.10/PC  '
             '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
             '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
             '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
             '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
             '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
             '-IC:/buildroot/prerequisites/x86_64-zlib-static/include -I.',
 'CXX': 'x86_64-w64-mingw32-c++',
 'DESTDIR': '',
 'DESTDIRFINAL': '/',
 'DESTDIRS': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt '
             'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
             'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9 '
             'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9/lib-dynload',
 'DESTLIB': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9',
 'DESTPATH': '',
 'DESTSHARED': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9/lib-dynload',
 'DFLAGS': '',
 'DIRMODE': 755,
 'DIST': 'README.rst ChangeLog configure configure.ac acconfig.h pyconfig.h.in '
         'Makefile.pre.in Include Lib Misc Ext-dummy',
 'DISTDIRS': 'Include Lib Misc Ext-dummy',
 'DISTFILES': 'README.rst ChangeLog configure configure.ac acconfig.h '
              'pyconfig.h.in Makefile.pre.in',
 'DLINCLDIR': '.',
 'DLLLIBRARY': 'libpython3.9.dll',
 'DOUBLE_IS_ARM_MIXED_ENDIAN_IEEE754': 0,
 'DOUBLE_IS_BIG_ENDIAN_IEEE754': 0,
 'DOUBLE_IS_LITTLE_ENDIAN_IEEE754': 1,
 'DTRACE': '',
 'DTRACE_DEPS': '\\',
 'DTRACE_HEADERS': '',
 'DTRACE_OBJS': '',
 'DYNLOADFILE': 'dynload_win.o',
 'ENABLE_IPV6': 0,
 'ENSUREPIP': 'no',
 'EXE': '.exe',
 'EXEMODE': 755,
 'EXPORTSFROM': '',
 'EXPORTSYMS': '',
 'EXTRATESTOPTS': '',
 'EXTRA_CFLAGS': '',
 'EXT_SUFFIX': '.cp39-mingw_x86_64_ucrt.pyd',
 'FILEMODE': 644,
 'FLOAT_WORDS_BIGENDIAN': 0,
 'FLOCK_NEEDS_LIBBSD': 0,
 'GETPGRP_HAVE_ARG': 0,
 'GITBRANCH': 'git --git-dir ../../../src/Python-3.9.10/.git name-rev '
              '--name-only HEAD',
 'GITTAG': 'git --git-dir ../../../src/Python-3.9.10/.git describe --all '
           '--always --dirty',
 'GITVERSION': 'git --git-dir ../../../src/Python-3.9.10/.git rev-parse '
               '--short HEAD',
 'GNULD': 'yes',
 'HAVE_ACCEPT4': 0,
 'HAVE_ACOSH': 1,
 'HAVE_ADDRINFO': 1,
 'HAVE_ALARM': 0,
 'HAVE_ALIGNED_REQUIRED': 0,
 'HAVE_ALLOCA_H': 0,
 'HAVE_ALTZONE': 0,
 'HAVE_ASINH': 1,
 'HAVE_ASM_TYPES_H': 0,
 'HAVE_ATANH': 1,
 'HAVE_BIND_TEXTDOMAIN_CODESET': 0,
 'HAVE_BLUETOOTH_BLUETOOTH_H': 0,
 'HAVE_BLUETOOTH_H': 0,
 'HAVE_BROKEN_MBSTOWCS': 0,
 'HAVE_BROKEN_NICE': 0,
 'HAVE_BROKEN_PIPE_BUF': 0,
 'HAVE_BROKEN_POLL': 0,
 'HAVE_BROKEN_POSIX_SEMAPHORES': 0,
 'HAVE_BROKEN_PTHREAD_SIGMASK': 0,
 'HAVE_BROKEN_SEM_GETVALUE': 0,
 'HAVE_BROKEN_UNSETENV': 0,
 'HAVE_BUILTIN_ATOMIC': 1,
 'HAVE_CHFLAGS': 0,
 'HAVE_CHOWN': 0,
 'HAVE_CHROOT': 0,
 'HAVE_CLOCK': 1,
 'HAVE_CLOCK_GETRES': 0,
 'HAVE_CLOCK_GETTIME': 0,
 'HAVE_CLOCK_SETTIME': 0,
 'HAVE_COMPUTED_GOTOS': 1,
 'HAVE_CONFSTR': 0,
 'HAVE_CONIO_H': 1,
 'HAVE_COPYSIGN': 1,
 'HAVE_COPY_FILE_RANGE': 0,
 'HAVE_CRYPT_H': 0,
 'HAVE_CRYPT_R': 0,
 'HAVE_CTERMID': 0,
 'HAVE_CTERMID_R': 0,
 'HAVE_CURSES_FILTER': 0,
 'HAVE_CURSES_H': 0,
 'HAVE_CURSES_HAS_KEY': 0,
 'HAVE_CURSES_IMMEDOK': 0,
 'HAVE_CURSES_IS_PAD': 0,
 'HAVE_CURSES_IS_TERM_RESIZED': 0,
 'HAVE_CURSES_RESIZETERM': 0,
 'HAVE_CURSES_RESIZE_TERM': 0,
 'HAVE_CURSES_SYNCOK': 0,
 'HAVE_CURSES_TYPEAHEAD': 0,
 'HAVE_CURSES_USE_ENV': 0,
 'HAVE_CURSES_WCHGAT': 0,
 'HAVE_DECL_ISFINITE': 1,
 'HAVE_DECL_ISINF': 1,
 'HAVE_DECL_ISNAN': 1,
 'HAVE_DECL_RTLD_DEEPBIND': 0,
 'HAVE_DECL_RTLD_GLOBAL': 0,
 'HAVE_DECL_RTLD_LAZY': 0,
 'HAVE_DECL_RTLD_LOCAL': 0,
 'HAVE_DECL_RTLD_MEMBER': 0,
 'HAVE_DECL_RTLD_NODELETE': 0,
 'HAVE_DECL_RTLD_NOLOAD': 0,
 'HAVE_DECL_RTLD_NOW': 0,
 'HAVE_DECL_TZNAME': 1,
 'HAVE_DEVICE_MACROS': 0,
 'HAVE_DEV_PTC': 0,
 'HAVE_DEV_PTMX': 0,
 'HAVE_DIRECT_H': 1,
 'HAVE_DIRENT_D_TYPE': 0,
 'HAVE_DIRENT_H': 1,
 'HAVE_DIRFD': 0,
 'HAVE_DLFCN_H': 0,
 'HAVE_DLOPEN': 0,
 'HAVE_DUP2': 1,
 'HAVE_DUP3': 0,
 'HAVE_DYLD_SHARED_CACHE_CONTAINS_PATH': 0,
 'HAVE_DYNAMIC_LOADING': 1,
 'HAVE_ENDIAN_H': 0,
 'HAVE_EPOLL': 0,
 'HAVE_EPOLL_CREATE1': 0,
 'HAVE_ERF': 1,
 'HAVE_ERFC': 1,
 'HAVE_ERRNO_H': 1,
 'HAVE_EXECV': 1,
 'HAVE_EXPLICIT_BZERO': 0,
 'HAVE_EXPLICIT_MEMSET': 0,
 'HAVE_EXPM1': 1,
 'HAVE_FACCESSAT': 0,
 'HAVE_FCHDIR': 0,
 'HAVE_FCHMOD': 0,
 'HAVE_FCHMODAT': 0,
 'HAVE_FCHOWN': 0,
 'HAVE_FCHOWNAT': 0,
 'HAVE_FCNTL_H': 1,
 'HAVE_FDATASYNC': 0,
 'HAVE_FDOPENDIR': 0,
 'HAVE_FDWALK': 0,
 'HAVE_FEXECVE': 0,
 'HAVE_FINITE': 1,
 'HAVE_FLOCK': 0,
 'HAVE_FORK': 0,
 'HAVE_FORKPTY': 0,
 'HAVE_FPATHCONF': 0,
 'HAVE_FSEEK64': 0,
 'HAVE_FSEEKO': 1,
 'HAVE_FSTATAT': 0,
 'HAVE_FSTATVFS': 0,
 'HAVE_FSYNC': 0,
 'HAVE_FTELL64': 0,
 'HAVE_FTELLO': 1,
 'HAVE_FTIME': 1,
 'HAVE_FTRUNCATE': 0,
 'HAVE_FUTIMENS': 0,
 'HAVE_FUTIMES': 0,
 'HAVE_FUTIMESAT': 0,
 'HAVE_GAI_STRERROR': 0,
 'HAVE_GAMMA': 0,
 'HAVE_GCC_ASM_FOR_MC68881': 0,
 'HAVE_GCC_ASM_FOR_X64': 1,
 'HAVE_GCC_ASM_FOR_X87': 1,
 'HAVE_GCC_UINT128_T': 1,
 'HAVE_GETADDRINFO': 0,
 'HAVE_GETC_UNLOCKED': 0,
 'HAVE_GETENTROPY': 0,
 'HAVE_GETGRGID_R': 0,
 'HAVE_GETGRNAM_R': 0,
 'HAVE_GETGROUPLIST': 0,
 'HAVE_GETGROUPS': 0,
 'HAVE_GETHOSTBYNAME': 0,
 'HAVE_GETHOSTBYNAME_R': 0,
 'HAVE_GETHOSTBYNAME_R_3_ARG': 0,
 'HAVE_GETHOSTBYNAME_R_5_ARG': 0,
 'HAVE_GETHOSTBYNAME_R_6_ARG': 0,
 'HAVE_GETITIMER': 0,
 'HAVE_GETLOADAVG': 0,
 'HAVE_GETLOGIN': 1,
 'HAVE_GETNAMEINFO': 0,
 'HAVE_GETPAGESIZE': 0,
 'HAVE_GETPEERNAME': 1,
 'HAVE_GETPGID': 0,
 'HAVE_GETPGRP': 0,
 'HAVE_GETPID': 1,
 'HAVE_GETPRIORITY': 0,
 'HAVE_GETPWENT': 0,
 'HAVE_GETPWNAM_R': 0,
 'HAVE_GETPWUID_R': 0,
 'HAVE_GETRANDOM': 0,
 'HAVE_GETRANDOM_SYSCALL': 0,
 'HAVE_GETRESGID': 0,
 'HAVE_GETRESUID': 0,
 'HAVE_GETSID': 0,
 'HAVE_GETSPENT': 0,
 'HAVE_GETSPNAM': 0,
 'HAVE_GETWD': 0,
 'HAVE_GLIBC_MEMMOVE_BUG': 0,
 'HAVE_GRP_H': 0,
 'HAVE_HSTRERROR': 0,
 'HAVE_HTOLE64': 0,
 'HAVE_HYPOT': 1,
 'HAVE_IEEEFP_H': 1,
 'HAVE_IF_NAMEINDEX': 0,
 'HAVE_INET_ATON': 0,
 'HAVE_INET_PTON': 1,
 'HAVE_INITGROUPS': 0,
 'HAVE_INTTYPES_H': 1,
 'HAVE_IO_H': 1,
 'HAVE_IPA_PURE_CONST_BUG': 0,
 'HAVE_KILL': 0,
 'HAVE_KILLPG': 0,
 'HAVE_KQUEUE': 0,
 'HAVE_LANGINFO_H': 0,
 'HAVE_LARGEFILE_SUPPORT': 1,
 'HAVE_LCHFLAGS': 0,
 'HAVE_LCHMOD': 0,
 'HAVE_LCHOWN': 0,
 'HAVE_LGAMMA': 1,
 'HAVE_LIBDL': 0,
 'HAVE_LIBDLD': 0,
 'HAVE_LIBIEEE': 0,
 'HAVE_LIBINTL_H': 0,
 'HAVE_LIBREADLINE': 1,
 'HAVE_LIBRESOLV': 0,
 'HAVE_LIBSENDFILE': 0,
 'HAVE_LIBUTIL_H': 0,
 'HAVE_LIBUUID': 0,
 'HAVE_LINK': 0,
 'HAVE_LINKAT': 0,
 'HAVE_LINUX_CAN_BCM_H': 0,
 'HAVE_LINUX_CAN_H': 0,
 'HAVE_LINUX_CAN_J1939_H': 0,
 'HAVE_LINUX_CAN_RAW_FD_FRAMES': 0,
 'HAVE_LINUX_CAN_RAW_H': 0,
 'HAVE_LINUX_CAN_RAW_JOIN_FILTERS': 0,
 'HAVE_LINUX_MEMFD_H': 0,
 'HAVE_LINUX_NETLINK_H': 0,
 'HAVE_LINUX_QRTR_H': 0,
 'HAVE_LINUX_RANDOM_H': 0,
 'HAVE_LINUX_TIPC_H': 0,
 'HAVE_LINUX_VM_SOCKETS_H': 0,
 'HAVE_LINUX_WAIT_H': 0,
 'HAVE_LOCKF': 0,
 'HAVE_LOG1P': 1,
 'HAVE_LOG2': 1,
 'HAVE_LONG_DOUBLE': 1,
 'HAVE_LSTAT': 0,
 'HAVE_LUTIMES': 0,
 'HAVE_MADVISE': 0,
 'HAVE_MAKEDEV': 0,
 'HAVE_MBRTOWC': 1,
 'HAVE_MEMFD_CREATE': 0,
 'HAVE_MEMORY_H': 1,
 'HAVE_MEMRCHR': 0,
 'HAVE_MKDIRAT': 0,
 'HAVE_MKFIFO': 0,
 'HAVE_MKFIFOAT': 0,
 'HAVE_MKNOD': 0,
 'HAVE_MKNODAT': 0,
 'HAVE_MKTIME': 1,
 'HAVE_MMAP': 0,
 'HAVE_MREMAP': 0,
 'HAVE_NCURSES_H': 0,
 'HAVE_NDIR_H': 0,
 'HAVE_NETPACKET_PACKET_H': 0,
 'HAVE_NET_IF_H': 0,
 'HAVE_NICE': 0,
 'HAVE_NON_UNICODE_WCHAR_T_REPRESENTATION': 0,
 'HAVE_OPENAT': 0,
 'HAVE_OPENPTY': 0,
 'HAVE_PATHCONF': 0,
 'HAVE_PAUSE': 0,
 'HAVE_PIPE2': 0,
 'HAVE_PLOCK': 0,
 'HAVE_POLL': 0,
 'HAVE_POLL_H': 0,
 'HAVE_POSIX_FADVISE': 0,
 'HAVE_POSIX_FALLOCATE': 0,
 'HAVE_POSIX_SPAWN': 0,
 'HAVE_POSIX_SPAWNP': 0,
 'HAVE_PREAD': 0,
 'HAVE_PREADV': 0,
 'HAVE_PREADV2': 0,
 'HAVE_PRLIMIT': 0,
 'HAVE_PROCESS_H': 1,
 'HAVE_PROTOTYPES': 1,
 'HAVE_PTHREAD_CONDATTR_SETCLOCK': 0,
 'HAVE_PTHREAD_DESTRUCTOR': 0,
 'HAVE_PTHREAD_GETCPUCLOCKID': 0,
 'HAVE_PTHREAD_H': 0,
 'HAVE_PTHREAD_INIT': 0,
 'HAVE_PTHREAD_KILL': 0,
 'HAVE_PTHREAD_SIGMASK': 0,
 'HAVE_PTY_H': 0,
 'HAVE_PWRITE': 0,
 'HAVE_PWRITEV': 0,
 'HAVE_PWRITEV2': 0,
 'HAVE_READLINK': 0,
 'HAVE_READLINKAT': 0,
 'HAVE_READV': 0,
 'HAVE_REALPATH': 0,
 'HAVE_RENAMEAT': 0,
 'HAVE_RL_APPEND_HISTORY': 1,
 'HAVE_RL_CATCH_SIGNAL': 1,
 'HAVE_RL_COMPLETION_APPEND_CHARACTER': 1,
 'HAVE_RL_COMPLETION_DISPLAY_MATCHES_HOOK': 1,
 'HAVE_RL_COMPLETION_MATCHES': 1,
 'HAVE_RL_COMPLETION_SUPPRESS_APPEND': 1,
 'HAVE_RL_PRE_INPUT_HOOK': 1,
 'HAVE_RL_RESIZE_TERMINAL': 1,
 'HAVE_ROUND': 1,
 'HAVE_RTPSPAWN': 0,
 'HAVE_SCHED_GET_PRIORITY_MAX': 0,
 'HAVE_SCHED_H': 0,
 'HAVE_SCHED_RR_GET_INTERVAL': 0,
 'HAVE_SCHED_SETAFFINITY': 0,
 'HAVE_SCHED_SETPARAM': 0,
 'HAVE_SCHED_SETSCHEDULER': 0,
 'HAVE_SEM_CLOCKWAIT': 0,
 'HAVE_SEM_GETVALUE': 0,
 'HAVE_SEM_OPEN': 0,
 'HAVE_SEM_TIMEDWAIT': 0,
 'HAVE_SEM_UNLINK': 0,
 'HAVE_SENDFILE': 0,
 'HAVE_SETEGID': 0,
 'HAVE_SETEUID': 0,
 'HAVE_SETGID': 0,
 'HAVE_SETGROUPS': 0,
 'HAVE_SETHOSTNAME': 0,
 'HAVE_SETITIMER': 0,
 'HAVE_SETLOCALE': 1,
 'HAVE_SETPGID': 0,
 'HAVE_SETPGRP': 0,
 'HAVE_SETPRIORITY': 0,
 'HAVE_SETREGID': 0,
 'HAVE_SETRESGID': 0,
 'HAVE_SETRESUID': 0,
 'HAVE_SETREUID': 0,
 'HAVE_SETSID': 0,
 'HAVE_SETUID': 0,
 'HAVE_SETVBUF': 1,
 'HAVE_SHADOW_H': 0,
 'HAVE_SHM_OPEN': 0,
 'HAVE_SHM_UNLINK': 0,
 'HAVE_SIGACTION': 0,
 'HAVE_SIGALTSTACK': 0,
 'HAVE_SIGFILLSET': 0,
 'HAVE_SIGINFO_T_SI_BAND': 0,
 'HAVE_SIGINTERRUPT': 0,
 'HAVE_SIGNAL_H': 1,
 'HAVE_SIGPENDING': 0,
 'HAVE_SIGRELSE': 0,
 'HAVE_SIGTIMEDWAIT': 0,
 'HAVE_SIGWAIT': 0,
 'HAVE_SIGWAITINFO': 0,
 'HAVE_SNPRINTF': 1,
 'HAVE_SOCKADDR_ALG': 0,
 'HAVE_SOCKADDR_SA_LEN': 0,
 'HAVE_SOCKADDR_STORAGE': 1,
 'HAVE_SOCKETPAIR': 0,
 'HAVE_SPAWN_H': 0,
 'HAVE_SSIZE_T': 1,
 'HAVE_STATVFS': 0,
 'HAVE_STAT_TV_NSEC': 0,
 'HAVE_STAT_TV_NSEC2': 0,
 'HAVE_STDARG_PROTOTYPES': 1,
 'HAVE_STDINT_H': 1,
 'HAVE_STDLIB_H': 1,
 'HAVE_STD_ATOMIC': 1,
 'HAVE_STRDUP': 1,
 'HAVE_STRFTIME': 1,
 'HAVE_STRINGS_H': 1,
 'HAVE_STRING_H': 1,
 'HAVE_STRLCPY': 0,
 'HAVE_STROPTS_H': 0,
 'HAVE_STRSIGNAL': 0,
 'HAVE_STRUCT_PASSWD_PW_GECOS': 0,
 'HAVE_STRUCT_PASSWD_PW_PASSWD': 0,
 'HAVE_STRUCT_STAT_ST_BIRTHTIME': 0,
 'HAVE_STRUCT_STAT_ST_BLKSIZE': 0,
 'HAVE_STRUCT_STAT_ST_BLOCKS': 0,
 'HAVE_STRUCT_STAT_ST_FLAGS': 0,
 'HAVE_STRUCT_STAT_ST_GEN': 0,
 'HAVE_STRUCT_STAT_ST_RDEV': 1,
 'HAVE_STRUCT_TM_TM_ZONE': 0,
 'HAVE_SYMLINK': 0,
 'HAVE_SYMLINKAT': 0,
 'HAVE_SYNC': 0,
 'HAVE_SYSCONF': 0,
 'HAVE_SYSEXITS_H': 0,
 'HAVE_SYS_AUDIOIO_H': 0,
 'HAVE_SYS_BSDTTY_H': 0,
 'HAVE_SYS_DEVPOLL_H': 0,
 'HAVE_SYS_DIR_H': 0,
 'HAVE_SYS_ENDIAN_H': 0,
 'HAVE_SYS_EPOLL_H': 0,
 'HAVE_SYS_EVENT_H': 0,
 'HAVE_SYS_FILE_H': 1,
 'HAVE_SYS_IOCTL_H': 0,
 'HAVE_SYS_KERN_CONTROL_H': 0,
 'HAVE_SYS_LOADAVG_H': 0,
 'HAVE_SYS_LOCK_H': 0,
 'HAVE_SYS_MEMFD_H': 0,
 'HAVE_SYS_MKDEV_H': 0,
 'HAVE_SYS_MMAN_H': 0,
 'HAVE_SYS_MODEM_H': 0,
 'HAVE_SYS_NDIR_H': 0,
 'HAVE_SYS_PARAM_H': 1,
 'HAVE_SYS_POLL_H': 0,
 'HAVE_SYS_RANDOM_H': 0,
 'HAVE_SYS_RESOURCE_H': 0,
 'HAVE_SYS_SELECT_H': 0,
 'HAVE_SYS_SENDFILE_H': 0,
 'HAVE_SYS_SOCKET_H': 0,
 'HAVE_SYS_STATVFS_H': 0,
 'HAVE_SYS_STAT_H': 1,
 'HAVE_SYS_SYSCALL_H': 0,
 'HAVE_SYS_SYSMACROS_H': 0,
 'HAVE_SYS_SYS_DOMAIN_H': 0,
 'HAVE_SYS_TERMIO_H': 0,
 'HAVE_SYS_TIMES_H': 0,
 'HAVE_SYS_TIME_H': 1,
 'HAVE_SYS_TYPES_H': 1,
 'HAVE_SYS_UIO_H': 0,
 'HAVE_SYS_UN_H': 0,
 'HAVE_SYS_UTSNAME_H': 0,
 'HAVE_SYS_WAIT_H': 0,
 'HAVE_SYS_XATTR_H': 0,
 'HAVE_TCGETPGRP': 0,
 'HAVE_TCSETPGRP': 0,
 'HAVE_TEMPNAM': 1,
 'HAVE_TERMIOS_H': 0,
 'HAVE_TERM_H': 0,
 'HAVE_TGAMMA': 1,
 'HAVE_THREAD_H': 0,
 'HAVE_TIMEGM': 0,
 'HAVE_TIMES': 0,
 'HAVE_TMPFILE': 1,
 'HAVE_TMPNAM': 1,
 'HAVE_TMPNAM_R': 0,
 'HAVE_TM_ZONE': 0,
 'HAVE_TRUNCATE': 0,
 'HAVE_TZNAME': 1,
 'HAVE_UCS4_TCL': 0,
 'HAVE_UNAME': 0,
 'HAVE_UNISTD_H': 1,
 'HAVE_UNLINKAT': 0,
 'HAVE_USABLE_WCHAR_T': 1,
 'HAVE_UTIL_H': 0,
 'HAVE_UTIMENSAT': 0,
 'HAVE_UTIMES': 0,
 'HAVE_UTIME_H': 1,
 'HAVE_UUID_CREATE': 0,
 'HAVE_UUID_ENC_BE': 0,
 'HAVE_UUID_GENERATE_TIME_SAFE': 0,
 'HAVE_UUID_H': 0,
 'HAVE_UUID_UUID_H': 0,
 'HAVE_WAIT3': 0,
 'HAVE_WAIT4': 0,
 'HAVE_WAITID': 0,
 'HAVE_WAITPID': 0,
 'HAVE_WCHAR_H': 1,
 'HAVE_WCSCOLL': 1,
 'HAVE_WCSFTIME': 1,
 'HAVE_WCSXFRM': 1,
 'HAVE_WMEMCMP': 1,
 'HAVE_WORKING_TZSET': 0,
 'HAVE_WRITEV': 0,
 'HAVE_WS2TCPIP_H': 1,
 'HAVE_X509_VERIFY_PARAM_SET1_HOST': 0,
 'HAVE_ZLIB_COPY': 1,
 'HAVE__GETPTY': 0,
 'HOST_GNU_TYPE': 'x86_64-w64-mingw32',
 'INCLDIRSTOMAKE': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                   'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                   'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/python3.9 '
                   'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/python3.9',
 'INCLUDEDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include',
 'INCLUDEPY': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/python3.9',
 'INSTALL': '/usr/bin/install -c',
 'INSTALL_DATA': '/usr/bin/install -c -m 644',
 'INSTALL_PROGRAM': '/usr/bin/install -c',
 'INSTALL_SCRIPT': '/usr/bin/install -c',
 'INSTALL_SHARED': '/usr/bin/install -c -m 755',
 'INSTSONAME': 'libpython3.9.dll.a',
 'IO_H': 'Modules/_io/_iomodule.h',
 'IO_OBJS': '\\',
 'LDCXXSHARED': 'x86_64-w64-mingw32-c++ -shared -Wl,--enable-auto-image-base',
 'LDFLAGS': '-pipe -fno-ident '
            '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
            '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
            '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
            '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
            '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'LDLIBRARY': 'libpython3.9.dll.a',
 'LDLIBRARYDIR': '',
 'LDSHARED': 'x86_64-w64-mingw32-gcc -shared -Wl,--enable-auto-image-base '
             '-pipe -fno-ident '
             '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
             '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
             '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
             '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
             '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'LDVERSION': '3.9',
 'LIBC': '',
 'LIBDEST': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9',
 'LIBDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'LIBFFI_INCLUDEDIR': '',
 'LIBM': '-lm',
 'LIBOBJDIR': 'Python/',
 'LIBOBJS': '',
 'LIBPC': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/pkgconfig',
 'LIBPL': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9/config-3.9',
 'LIBPYTHON': '-lpython3.9',
 'LIBRARY': 'libpython3.9.a',
 'LIBRARY_OBJS': '\\',
 'LIBRARY_OBJS_OMIT_FROZEN': '\\',
 'LIBS': '-lm -lversion -lshlwapi',
 'LIBSUBDIRS': 'tkinter tkinter/test tkinter/test/test_tkinter \\',
 'LINKCC': 'x86_64-w64-mingw32-gcc',
 'LINKFORSHARED': '-Wl,--stack,2000000',
 'LIPO_32BIT_FLAGS': '',
 'LIPO_INTEL64_FLAGS': '',
 'LLVM_PROF_ERR': 'no',
 'LLVM_PROF_FILE': '',
 'LLVM_PROF_MERGER': 'true',
 'LN': 'ln',
 'LOCALMODLIBS': '-lws2_32',
 'MACHDEP': 'win32',
 'MACHDEP_OBJS': 'PC/dl_nt.o',
 'MACHDESTLIB': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9',
 'MACOSX_DEPLOYMENT_TARGET': '',
 'MAINCC': 'x86_64-w64-mingw32-gcc',
 'MAJOR_IN_MKDEV': 0,
 'MAJOR_IN_SYSMACROS': 0,
 'MAKESETUP': '../../../src/Python-3.9.10/Modules/makesetup',
 'MANDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/share/man',
 'MKDIR_P': '/usr/bin/mkdir -p',
 'MODBUILT_NAMES': 'nt  winreg  msvcrt  _winapi  errno  _sre  _codecs  '
                   '_weakref  _functools  _operator  _collections  _abc  '
                   'itertools  atexit  _signal  _stat  time  _thread  _locale  '
                   '_io  faulthandler  _tracemalloc  _peg_parser  _symtable  '
                   'xxsubtype',
 'MODDISABLED_NAMES': '',
 'MODLIBS': '-lws2_32',
 'MODOBJS': 'Modules/posixmodule.o  Modules/winreg.o  Modules/msvcrtmodule.o  '
            'Modules/_winapi.o  Modules/errnomodule.o  Modules/_sre.o  '
            'Modules/_codecsmodule.o  Modules/_weakref.o  '
            'Modules/_functoolsmodule.o  Modules/_operator.o  '
            'Modules/_collectionsmodule.o  Modules/_abc.o  '
            'Modules/itertoolsmodule.o  Modules/atexitmodule.o  '
            'Modules/signalmodule.o  Modules/_stat.o  Modules/timemodule.o  '
            'Modules/_threadmodule.o  Modules/_localemodule.o  '
            'Modules/_iomodule.o Modules/iobase.o Modules/fileio.o '
            'Modules/bytesio.o Modules/bufferedio.o Modules/textio.o '
            'Modules/stringio.o Modules/winconsoleio.o  '
            'Modules/faulthandler.o  Modules/_tracemalloc.o  '
            'Modules/_peg_parser.o  Modules/symtablemodule.o  '
            'Modules/xxsubtype.o',
 'MODULE_OBJS': '\\',
 'MULTIARCH': '',
 'MULTIARCH_CPPFLAGS': '',
 'MVWDELCH_IS_EXPRESSION': 0,
 'NCURSESW_INCLUDEDIR': '',
 'NO_AS_NEEDED': '-Wl,--no-as-needed',
 'NT_THREADS': 1,
 'OBJECT_OBJS': '\\',
 'OPENSSL_INCLUDES': '',
 'OPENSSL_LDFLAGS': '',
 'OPENSSL_LIBS': '',
 'OPT': '-DNDEBUG -g -fwrapv -O3 -Wall',
 'OTHER_LIBTOOL_OPT': '',
 'PACKAGE_BUGREPORT': 0,
 'PACKAGE_NAME': 0,
 'PACKAGE_STRING': 0,
 'PACKAGE_TARNAME': 0,
 'PACKAGE_URL': 0,
 'PACKAGE_VERSION': 0,
 'PARSER_HEADERS': '\\',
 'PARSER_OBJS': '\\ \\ Parser/myreadline.o Parser/parsetok.o '
                'Parser/tokenizer.o',
 'PEGEN_HEADERS': '\\',
 'PEGEN_OBJS': '\\',
 'PGO_PROF_GEN_FLAG': '-fprofile-generate',
 'PGO_PROF_USE_FLAG': '-fprofile-use -fprofile-correction',
 'PLATLIBDIR': 'lib',
 'POBJS': '\\',
 'POSIX_SEMAPHORES_NOT_ENABLED': 1,
 'PROFILE_TASK': '-m test --pgo',
 'PTHREAD_KEY_T_IS_COMPATIBLE_WITH_INT': 0,
 'PTHREAD_SYSTEM_SCHED_SUPPORTED': 0,
 'PURIFY': '',
 'PY3LIBRARY': '',
 'PYD_PLATFORM_TAG': 'mingw_x86_64_ucrt',
 'PYLONG_BITS_IN_DIGIT': 0,
 'PYTHON': 'python.exe',
 'PYTHONFRAMEWORK': '',
 'PYTHONFRAMEWORKDIR': 'no-framework',
 'PYTHONFRAMEWORKINSTALLDIR': '',
 'PYTHONFRAMEWORKPREFIX': '',
 'PYTHONPATH': '',
 'PYTHON_FOR_BUILD': './python.exe -E',
 'PYTHON_HEADERS': '\\',
 'PYTHON_OBJS': '\\',
 'PY_BUILTIN_HASHLIB_HASHES': '"md5,sha1,sha256,sha512,sha3,blake2"',
 'PY_BUILTIN_MODULE_CFLAGS': '-Wno-unused-result -Wsign-compare -DNDEBUG -g '
                             '-fwrapv -O3 -Wall -O2 -pipe -fno-ident '
                             '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                             '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                             '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                             '-D__USE_MINGW_ANSI_STDIO=1 -std=c99 -Wextra '
                             '-Wno-unused-result -Wno-unused-parameter '
                             '-Wno-missing-field-initializers '
                             '-Wstrict-prototypes '
                             '-Werror=implicit-function-declaration '
                             '-fvisibility=hidden -D_WIN32_WINNT=0x0601 '
                             '-DMS_DLL_ID=\'"3.9"\' -fprofile-use '
                             '-fprofile-correction '
                             '-I../../../src/Python-3.9.10/Include/internal '
                             '-IObjects -IInclude -IPython -I. '
                             '-I../../../src/Python-3.9.10/Include '
                             '-I../../../src/Python-3.9.10/PC  '
                             '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                             '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                             '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                             '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                             '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                             '-IC:/buildroot/prerequisites/x86_64-zlib-static/include '
                             '-I. -DPy_BUILD_CORE_BUILTIN',
 'PY_CFLAGS': '-Wno-unused-result -Wsign-compare -DNDEBUG -g -fwrapv -O3 -Wall '
              '-O2 -pipe -fno-ident '
              '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
              '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
              '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
              '-D__USE_MINGW_ANSI_STDIO=1',
 'PY_CFLAGS_NODIST': '-std=c99 -Wextra -Wno-unused-result '
                     '-Wno-unused-parameter -Wno-missing-field-initializers '
                     '-Wstrict-prototypes '
                     '-Werror=implicit-function-declaration '
                     '-fvisibility=hidden -D_WIN32_WINNT=0x0601 '
                     '-DMS_DLL_ID=\'"3.9"\' -fprofile-use -fprofile-correction '
                     '-I../../../src/Python-3.9.10/Include/internal',
 'PY_COERCE_C_LOCALE': 0,
 'PY_CORE_CFLAGS': '-Wno-unused-result -Wsign-compare -DNDEBUG -g -fwrapv -O3 '
                   '-Wall -O2 -pipe -fno-ident '
                   '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                   '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                   '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                   '-D__USE_MINGW_ANSI_STDIO=1 -std=c99 -Wextra '
                   '-Wno-unused-result -Wno-unused-parameter '
                   '-Wno-missing-field-initializers -Wstrict-prototypes '
                   '-Werror=implicit-function-declaration -fvisibility=hidden '
                   '-D_WIN32_WINNT=0x0601 -DMS_DLL_ID=\'"3.9"\' -fprofile-use '
                   '-fprofile-correction '
                   '-I../../../src/Python-3.9.10/Include/internal -IObjects '
                   '-IInclude -IPython -I. '
                   '-I../../../src/Python-3.9.10/Include '
                   '-I../../../src/Python-3.9.10/PC  '
                   '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                   '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                   '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                   '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                   '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                   '-IC:/buildroot/prerequisites/x86_64-zlib-static/include '
                   '-I. -DPy_BUILD_CORE',
 'PY_CORE_LDFLAGS': '-pipe -fno-ident '
                    '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
                    '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
                    '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
                    '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
                    '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'PY_CPPFLAGS': '-IObjects -IInclude -IPython -I. '
                '-I../../../src/Python-3.9.10/Include '
                '-I../../../src/Python-3.9.10/PC  '
                '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                '-IC:/buildroot/prerequisites/x86_64-zlib-static/include -I.',
 'PY_FORMAT_SIZE_T': '"z"',
 'PY_LDFLAGS': '-pipe -fno-ident '
               '-L/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib '
               '-L/c/buildroot/prerequisites/x86_64-zlib-static/lib '
               '-L/c/buildroot/prerequisites/x86_64-w64-mingw32-static/lib '
               '-LC:/buildroot/prerequisites/x86_64-zlib-static/lib '
               '-LC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'PY_LDFLAGS_NODIST': '',
 'PY_SSL_DEFAULT_CIPHERS': 1,
 'PY_SSL_DEFAULT_CIPHER_STRING': 0,
 'PY_STDMODULE_CFLAGS': '-Wno-unused-result -Wsign-compare -DNDEBUG -g -fwrapv '
                        '-O3 -Wall -O2 -pipe -fno-ident '
                        '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                        '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                        '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                        '-D__USE_MINGW_ANSI_STDIO=1 -std=c99 -Wextra '
                        '-Wno-unused-result -Wno-unused-parameter '
                        '-Wno-missing-field-initializers -Wstrict-prototypes '
                        '-Werror=implicit-function-declaration '
                        '-fvisibility=hidden -D_WIN32_WINNT=0x0601 '
                        '-DMS_DLL_ID=\'"3.9"\' -fprofile-use '
                        '-fprofile-correction '
                        '-I../../../src/Python-3.9.10/Include/internal '
                        '-IObjects -IInclude -IPython -I. '
                        '-I../../../src/Python-3.9.10/Include '
                        '-I../../../src/Python-3.9.10/PC  '
                        '-I/c/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                        '-I/c/buildroot/prerequisites/x86_64-zlib-static/include '
                        '-I/c/buildroot/prerequisites/x86_64-w64-mingw32-static/include '
                        '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include '
                        '-IC:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/include/ncursesw '
                        '-IC:/buildroot/prerequisites/x86_64-zlib-static/include '
                        '-I.',
 'Py_DEBUG': 0,
 'Py_ENABLE_SHARED': 1,
 'Py_HASH_ALGORITHM': 0,
 'Py_TRACE_REFS': 0,
 'QUICKTESTOPTS': '-x test_subprocess test_io test_lib2to3 \\',
 'RCFLAGS': '-DFIELD3=10150 -O COFF --target=pe-x86-64',
 'READELF': 'readelf',
 'RESSRCDIR': 'Mac/Resources/framework',
 'RETSIGTYPE': 'void',
 'RUNSHARED': '',
 'SCRIPTDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib',
 'SETPGRP_HAVE_ARG': 0,
 'SHELL': '/bin/sh',
 'SHLIBS': '-lm -lversion -lshlwapi',
 'SHLIB_SUFFIX': '.pyd',
 'SHM_NEEDS_LIBRT': 0,
 'SIGNED_RIGHT_SHIFT_ZERO_FILLS': 0,
 'SITEPATH': '',
 'SIZEOF_DOUBLE': 8,
 'SIZEOF_FLOAT': 4,
 'SIZEOF_FPOS_T': 8,
 'SIZEOF_INT': 4,
 'SIZEOF_LONG': 4,
 'SIZEOF_LONG_DOUBLE': 16,
 'SIZEOF_LONG_LONG': 8,
 'SIZEOF_OFF_T': 8,
 'SIZEOF_PID_T': 8,
 'SIZEOF_PTHREAD_KEY_T': 0,
 'SIZEOF_PTHREAD_T': 0,
 'SIZEOF_SHORT': 2,
 'SIZEOF_SIZE_T': 8,
 'SIZEOF_TIME_T': 8,
 'SIZEOF_UINTPTR_T': 8,
 'SIZEOF_VOID_P': 8,
 'SIZEOF_WCHAR_T': 2,
 'SIZEOF__BOOL': 1,
 'SOABI': 'cpython-39',
 'SRCDIRS': 'Parser Parser/pegen Objects Python Modules Modules/_io Programs '
            'PC',
 'SRC_GDB_HOOKS': '../../../src/Python-3.9.10/Tools/gdb/libpython.py',
 'STDC_HEADERS': 1,
 'STRICT_SYSV_CURSES': "/* Don't use ncurses extensions */",
 'STRIPFLAG': '-s',
 'SUBDIRS': '',
 'SUBDIRSTOO': 'Include Lib Misc',
 'SYSLIBS': '-lm',
 'SYS_SELECT_WITH_SYS_TIME': 0,
 'TCLTK_INCLUDES': '',
 'TCLTK_LIBS': '',
 'TESTOPTS': '',
 'TESTPATH': '',
 'TESTPYTHON': './python.exe',
 'TESTPYTHONOPTS': '',
 'TESTRUNNER': './python.exe '
               '../../../src/Python-3.9.10/Tools/scripts/run_tests.py',
 'TESTTIMEOUT': 1200,
 'TIMEMODULE_LIB': 0,
 'TIME_WITH_SYS_TIME': 1,
 'TM_IN_SYS_TIME': 0,
 'TZPATH': '/usr/share/zoneinfo:/usr/lib/zoneinfo:/usr/share/lib/zoneinfo:/etc/zoneinfo',
 'UNICODE_DEPS': '\\',
 'UNIVERSALSDK': '',
 'UPDATE_FILE': 'python3 '
                '../../../src/Python-3.9.10/Tools/scripts/update_file.py',
 'USE_COMPUTED_GOTOS': 0,
 'VENVLAUNCHERDIR': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/lib/python3.9/venv/scripts/nt',
 'VERSION': '3.9',
 'VPATH': 'C:/buildroot/src/Python-3.9.10',
 'VPATH_b2h': 'C:/buildroot/src/Python-3.9.10',
 'WINDOW_HAS_FLAGS': 0,
 'WINDRES': 'windres',
 'WITH_DECIMAL_CONTEXTVAR': 1,
 'WITH_DOC_STRINGS': 1,
 'WITH_DTRACE': 0,
 'WITH_DYLD': 0,
 'WITH_LIBINTL': 0,
 'WITH_NEXT_FRAMEWORK': 0,
 'WITH_PYMALLOC': 1,
 'WITH_VALGRIND': 0,
 'X87_DOUBLE_ROUNDING': 0,
 'XMLLIBSUBDIRS': 'xml xml/dom xml/etree xml/parsers xml/sax',
 'abs_builddir': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/build/Python-3.9.10',
 'abs_builddir_b2h': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/build/Python-3.9.10',
 'abs_srcdir': 'C:/buildroot/src/Python-3.9.10',
 'abs_srcdir_b2h': 'C:/buildroot/src/Python-3.9.10',
 'datarootdir': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt/share',
 'exec_prefix': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt',
 'prefix': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt',
 'prefix_b2h': 'C:/buildroot/x86_64-1320-win32-seh-ucrt-rt_v11-rev0/mingw64/opt',
 'srcdir': 'C:/buildroot/src/Python-3.9.10',
 'srcdir_b2h': 'C:/buildroot/src/Python-3.9.10'}


keys_to_replace = [
    'BINDIR', 'BINLIBDEST', 'CONFINCLUDEDIR',
    'CONFINCLUDEPY', 'DESTDIRS', 'DESTLIB', 'DESTSHARED',
    'INCLDIRSTOMAKE', 'INCLUDEDIR', 'INCLUDEPY',
    'LIBDEST', 'LIBDIR', 'LIBPC', 'LIBPL', 'MACHDESTLIB',
    'MANDIR', 'SCRIPTDIR', 'datarootdir', 'exec_prefix',
    'TZPATH',
]

prefix = build_time_vars['BINDIR'][:-4]

for key in keys_to_replace:
    value = build_time_vars[key]
    build_time_vars[key] = value.replace(prefix, sys.prefix)
