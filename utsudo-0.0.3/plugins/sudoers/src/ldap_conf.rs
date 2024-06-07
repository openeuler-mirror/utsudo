/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    improper_ctypes,
    unused_variables,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//crate
use crate::common::*;
use libc::strdup;
//include extern C
extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn ber_set_option(
        item: *mut libc::c_void,
        option: libc::c_int,
        invalue: *const libc::c_void,
    ) -> libc::c_int;
    fn ldap_set_option(
        ld: *mut LDAP,
        option: libc::c_int,
        invalue: *const libc::c_void,
    ) -> libc::c_int;
    fn ldap_err2string(err: libc::c_int) -> *mut libc::c_char;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut path_ldap_conf: *const libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    static mut path_ldap_secret: *const libc::c_char;
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn base64_decode(str: *const libc::c_char, dst: *mut libc::c_uchar, dsize: size_t) -> size_t;
}
//be used in ldap.rs structs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_table {
    pub conf_str: *const libc::c_char,
    pub type_0: libc::c_int,
    pub opt_val: libc::c_int,
    pub valp: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config {
    pub port: libc::c_int,
    pub version: libc::c_int,
    pub debug: libc::c_int,
    pub ldap_debug: libc::c_int,
    pub tls_checkpeer: libc::c_int,
    pub tls_reqcert: libc::c_int,
    pub timelimit: libc::c_int,
    pub timeout: libc::c_int,
    pub bind_timelimit: libc::c_int,
    pub use_sasl: libc::c_int,
    pub rootuse_sasl: libc::c_int,
    pub ssl_mode: libc::c_int,
    pub timed: libc::c_int,
    pub deref: libc::c_int,
    pub host: *mut libc::c_char,
    pub uri: ldap_config_str_list,
    pub binddn: *mut libc::c_char,
    pub bindpw: *mut libc::c_char,
    pub rootbinddn: *mut libc::c_char,
    pub base: ldap_config_str_list,
    pub netgroup_base: ldap_config_str_list,
    pub search_filter: *mut libc::c_char,
    pub netgroup_search_filter: *mut libc::c_char,
    pub ssl: *mut libc::c_char,
    pub tls_cacertfile: *mut libc::c_char,
    pub tls_cacertdir: *mut libc::c_char,
    pub tls_random_file: *mut libc::c_char,
    pub tls_cipher_suite: *mut libc::c_char,
    pub tls_certfile: *mut libc::c_char,
    pub tls_keyfile: *mut libc::c_char,
    pub tls_keypw: *mut libc::c_char,
    pub sasl_mech: *mut libc::c_char,
    pub sasl_auth_id: *mut libc::c_char,
    pub rootsasl_auth_id: *mut libc::c_char,
    pub sasl_secprops: *mut libc::c_char,
    pub krb5_ccname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str {
    pub entries: ldap_config_str_mid,
    pub val: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str_mid {
    pub stqe_next: *mut ldap_config_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_config_str_list {
    pub stqh_first: *mut ldap_config_str,
    pub stqh_last: *mut *mut ldap_config_str,
}
//constants && init
pub const CONF_DEREF_VAL: libc::c_int = 5;
pub const CONF_REQCERT_VAL: libc::c_int = 6;
pub const CONF_BOOL: libc::c_int = 0;
pub const CONF_INT: libc::c_int = 1;
pub const CONF_STR: libc::c_int = 2;
pub const CONF_LIST_STR: libc::c_int = 4;
pub const PARSELN_COMM_BOL: libc::c_int = 1;
pub const PARSELN_CONT_IGN: libc::c_int = 2;
pub const LDAP_OPT_X_TLS_NEVER: libc::c_int = 0;
pub const LDAP_OPT_X_TLS_ALLOW: libc::c_int = 3;
pub const LDAP_OPT_X_TLS_TRY: libc::c_int = 4;
pub const LDAP_OPT_X_TLS_HARD: libc::c_int = 1;
pub const LDAP_OPT_X_TLS_DEMAND: libc::c_int = 2;
pub const SUDO_LDAP_STARTTLS: libc::c_int = 2;
pub const SUDO_LDAP_SSL: libc::c_int = 1;
pub const LDAPS_PORT: libc::c_int = 636;
pub const LDAP_PORT: libc::c_int = 389;
pub const LDAP_SUCCESS: libc::c_int = 0;
pub const LDAP_OPT_SUCCESS: libc::c_int = 0;
pub const LBER_OPT_DEBUG_LEVEL: libc::c_int = 2;
pub const LDAP_OPT_TIMEOUT: libc::c_int = 20482;
pub const LDAP_OPT_NETWORK_TIMEOUT: libc::c_int = 20485;
pub const LDAP_OPT_X_TLS: libc::c_int = 24576;
#[no_mangle]
pub static mut ldap_conf: ldap_config = ldap_config {
    port: 0,
    version: 0,
    debug: 0,
    ldap_debug: 0,
    tls_checkpeer: 0,
    tls_reqcert: 0,
    timelimit: 0,
    timeout: 0,
    bind_timelimit: 0,
    use_sasl: 0,
    rootuse_sasl: 0,
    ssl_mode: 0,
    timed: 0,
    deref: 0,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    uri: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    binddn: 0 as *const libc::c_char as *mut libc::c_char,
    bindpw: 0 as *const libc::c_char as *mut libc::c_char,
    rootbinddn: 0 as *const libc::c_char as *mut libc::c_char,
    base: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    netgroup_base: ldap_config_str_list {
        stqh_first: 0 as *const ldap_config_str as *mut ldap_config_str,
        stqh_last: 0 as *const *mut ldap_config_str as *mut *mut ldap_config_str,
    },
    search_filter: 0 as *const libc::c_char as *mut libc::c_char,
    netgroup_search_filter: 0 as *const libc::c_char as *mut libc::c_char,
    ssl: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cacertfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cacertdir: 0 as *const libc::c_char as *mut libc::c_char,
    tls_random_file: 0 as *const libc::c_char as *mut libc::c_char,
    tls_cipher_suite: 0 as *const libc::c_char as *mut libc::c_char,
    tls_certfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_keyfile: 0 as *const libc::c_char as *mut libc::c_char,
    tls_keypw: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_mech: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_auth_id: 0 as *const libc::c_char as *mut libc::c_char,
    rootsasl_auth_id: 0 as *const libc::c_char as *mut libc::c_char,
    sasl_secprops: 0 as *const libc::c_char as *mut libc::c_char,
    krb5_ccname: 0 as *const libc::c_char as *mut libc::c_char,
};


static mut ldap_conf_global: [ldap_config_table; 31] = [ldap_config_table {
    conf_str: 0 as *const libc::c_char,
    type_0: 0,
    opt_val: 0,
    valp: 0 as *mut libc::c_void,
}; 31];
static mut ldap_conf_conn: [ldap_config_table; 8] = [ldap_config_table {
    conf_str: 0 as *const libc::c_char,
    type_0: 0,
    opt_val: 0,
    valp: 0 as *mut libc::c_void,
}; 8];
//pub static mut ldap_conf_global : [ldap_config_table;31] = unsafe {
//pub static mut ldap_conf_global : [ldap_config_table;31] =
unsafe extern "C" fn run_static_initializers() {
    ldap_conf_global = [
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_debug\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.debug as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"host\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.host as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"port\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.port as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"ssl\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.ssl as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sslpath\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.tls_certfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"uri\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.uri as *mut ldap_config_str_list as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"debug\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x5001 as libc::c_int,
                valp: &mut ldap_conf.ldap_debug as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_checkpeer\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: 0x6006 as libc::c_int,
                valp: &mut ldap_conf.tls_checkpeer as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_reqcert\0" as *const u8 as *const libc::c_char,
                type_0: 6 as libc::c_int,
                opt_val: 0x6006 as libc::c_int,
                valp: &mut ldap_conf.tls_reqcert as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacertfile\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6002 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacert\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6002 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cacertdir\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6003 as libc::c_int,
                valp: &mut ldap_conf.tls_cacertdir as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_randfile\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6009 as libc::c_int,
                valp: &mut ldap_conf.tls_random_file as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_ciphers\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6008 as libc::c_int,
                valp: &mut ldap_conf.tls_cipher_suite as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_cert\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6004 as libc::c_int,
                valp: &mut ldap_conf.tls_certfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"tls_key\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6005 as libc::c_int,
                valp: &mut ldap_conf.tls_keyfile as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"binddn\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.binddn as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"bindpw\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bindpw as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootbinddn\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootbinddn as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_base\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.base as *mut ldap_config_str_list as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_timed\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.timed as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sudoers_search_filter\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.search_filter as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"netgroup_base\0" as *const u8 as *const libc::c_char,
                type_0: 4 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.netgroup_base as *mut ldap_config_str_list
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"netgroup_search_filter\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.netgroup_search_filter as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"use_sasl\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.use_sasl as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_mech\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.sasl_mech as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_auth_id\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.sasl_auth_id as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootuse_sasl\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootuse_sasl as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"rootsasl_auth_id\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.rootsasl_auth_id as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"krb5_ccname\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.krb5_ccname as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: 0 as *const libc::c_char,
                type_0: 0,
                opt_val: 0,
                valp: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    //}
    
 //#[no_mangle]
    //pub static mut ldap_conf_conn : [ldap_config_table;8] = unsafe {
    //unsafe extern "C" fn run_static_initializers() {
    ldap_conf_conn = [
        //pub static mut ldap_conf_conn : [ldap_config_table;8] = unsafe {
        //    [
        {
            let mut init = ldap_config_table {
                conf_str: b"ldap_version\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x11 as libc::c_int,
                valp: &mut ldap_conf.version as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"bind_timelimit\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bind_timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"network_timeout\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.bind_timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"timelimit\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: 0x4 as libc::c_int,
                valp: &mut ldap_conf.timelimit as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"timeout\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                opt_val: -(1 as libc::c_int),
                valp: &mut ldap_conf.timeout as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"deref\0" as *const u8 as *const libc::c_char,
                type_0: 5 as libc::c_int,
                opt_val: 0x2 as libc::c_int,
                valp: &mut ldap_conf.deref as *mut libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: b"sasl_secprops\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                opt_val: 0x6106 as libc::c_int,
                valp: &mut ldap_conf.sasl_secprops as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = ldap_config_table {
                conf_str: 0 as *const libc::c_char,
                type_0: 0,
                opt_val: 0,
                valp: 0 as *mut libc::c_void,
            };
            init
        },
    ];
}








