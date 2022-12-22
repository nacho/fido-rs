/* automatically generated by rust-bindgen 0.63.0 */

pub const FIDO_ERR_SUCCESS: i32 = 0;
pub const FIDO_ERR_INVALID_COMMAND: i32 = 1;
pub const FIDO_ERR_INVALID_PARAMETER: i32 = 2;
pub const FIDO_ERR_INVALID_LENGTH: i32 = 3;
pub const FIDO_ERR_INVALID_SEQ: i32 = 4;
pub const FIDO_ERR_TIMEOUT: i32 = 5;
pub const FIDO_ERR_CHANNEL_BUSY: i32 = 6;
pub const FIDO_ERR_LOCK_REQUIRED: i32 = 10;
pub const FIDO_ERR_INVALID_CHANNEL: i32 = 11;
pub const FIDO_ERR_CBOR_UNEXPECTED_TYPE: i32 = 17;
pub const FIDO_ERR_INVALID_CBOR: i32 = 18;
pub const FIDO_ERR_MISSING_PARAMETER: i32 = 20;
pub const FIDO_ERR_LIMIT_EXCEEDED: i32 = 21;
pub const FIDO_ERR_UNSUPPORTED_EXTENSION: i32 = 22;
pub const FIDO_ERR_FP_DATABASE_FULL: i32 = 23;
pub const FIDO_ERR_LARGEBLOB_STORAGE_FULL: i32 = 24;
pub const FIDO_ERR_CREDENTIAL_EXCLUDED: i32 = 25;
pub const FIDO_ERR_PROCESSING: i32 = 33;
pub const FIDO_ERR_INVALID_CREDENTIAL: i32 = 34;
pub const FIDO_ERR_USER_ACTION_PENDING: i32 = 35;
pub const FIDO_ERR_OPERATION_PENDING: i32 = 36;
pub const FIDO_ERR_NO_OPERATIONS: i32 = 37;
pub const FIDO_ERR_UNSUPPORTED_ALGORITHM: i32 = 38;
pub const FIDO_ERR_OPERATION_DENIED: i32 = 39;
pub const FIDO_ERR_KEY_STORE_FULL: i32 = 40;
pub const FIDO_ERR_NOT_BUSY: i32 = 41;
pub const FIDO_ERR_NO_OPERATION_PENDING: i32 = 42;
pub const FIDO_ERR_UNSUPPORTED_OPTION: i32 = 43;
pub const FIDO_ERR_INVALID_OPTION: i32 = 44;
pub const FIDO_ERR_KEEPALIVE_CANCEL: i32 = 45;
pub const FIDO_ERR_NO_CREDENTIALS: i32 = 46;
pub const FIDO_ERR_USER_ACTION_TIMEOUT: i32 = 47;
pub const FIDO_ERR_NOT_ALLOWED: i32 = 48;
pub const FIDO_ERR_PIN_INVALID: i32 = 49;
pub const FIDO_ERR_PIN_BLOCKED: i32 = 50;
pub const FIDO_ERR_PIN_AUTH_INVALID: i32 = 51;
pub const FIDO_ERR_PIN_AUTH_BLOCKED: i32 = 52;
pub const FIDO_ERR_PIN_NOT_SET: i32 = 53;
pub const FIDO_ERR_PIN_REQUIRED: i32 = 54;
pub const FIDO_ERR_PIN_POLICY_VIOLATION: i32 = 55;
pub const FIDO_ERR_PIN_TOKEN_EXPIRED: i32 = 56;
pub const FIDO_ERR_REQUEST_TOO_LARGE: i32 = 57;
pub const FIDO_ERR_ACTION_TIMEOUT: i32 = 58;
pub const FIDO_ERR_UP_REQUIRED: i32 = 59;
pub const FIDO_ERR_UV_BLOCKED: i32 = 60;
pub const FIDO_ERR_UV_INVALID: i32 = 63;
pub const FIDO_ERR_UNAUTHORIZED_PERM: i32 = 64;
pub const FIDO_ERR_ERR_OTHER: i32 = 127;
pub const FIDO_ERR_SPEC_LAST: i32 = 223;
pub const FIDO_OK: i32 = 0;
pub const FIDO_ERR_TX: i32 = -1;
pub const FIDO_ERR_RX: i32 = -2;
pub const FIDO_ERR_RX_NOT_CBOR: i32 = -3;
pub const FIDO_ERR_RX_INVALID_CBOR: i32 = -4;
pub const FIDO_ERR_INVALID_PARAM: i32 = -5;
pub const FIDO_ERR_INVALID_SIG: i32 = -6;
pub const FIDO_ERR_INVALID_ARGUMENT: i32 = -7;
pub const FIDO_ERR_USER_PRESENCE_REQUIRED: i32 = -8;
pub const FIDO_ERR_INTERNAL: i32 = -9;
pub const FIDO_ERR_NOTFOUND: i32 = -10;
pub const FIDO_ERR_COMPRESS: i32 = -11;
pub const CTAP_AUTHDATA_USER_PRESENT: i32 = 1;
pub const CTAP_AUTHDATA_USER_VERIFIED: i32 = 4;
pub const CTAP_AUTHDATA_ATT_CRED: i32 = 64;
pub const CTAP_AUTHDATA_EXT_DATA: i32 = 128;
pub const CTAP_CMD_PING: i32 = 1;
pub const CTAP_CMD_MSG: i32 = 3;
pub const CTAP_CMD_LOCK: i32 = 4;
pub const CTAP_CMD_INIT: i32 = 6;
pub const CTAP_CMD_WINK: i32 = 8;
pub const CTAP_CMD_CBOR: i32 = 16;
pub const CTAP_CMD_CANCEL: i32 = 17;
pub const CTAP_KEEPALIVE: i32 = 59;
pub const CTAP_FRAME_INIT: i32 = 128;
pub const CTAP_CBOR_MAKECRED: i32 = 1;
pub const CTAP_CBOR_ASSERT: i32 = 2;
pub const CTAP_CBOR_GETINFO: i32 = 4;
pub const CTAP_CBOR_CLIENT_PIN: i32 = 6;
pub const CTAP_CBOR_RESET: i32 = 7;
pub const CTAP_CBOR_NEXT_ASSERT: i32 = 8;
pub const CTAP_CBOR_LARGEBLOB: i32 = 12;
pub const CTAP_CBOR_CONFIG: i32 = 13;
pub const CTAP_CBOR_BIO_ENROLL_PRE: i32 = 64;
pub const CTAP_CBOR_CRED_MGMT_PRE: i32 = 65;
pub const CTAP_PIN_PROTOCOL1: i32 = 1;
pub const CTAP_PIN_PROTOCOL2: i32 = 2;
pub const U2F_CMD_REGISTER: i32 = 1;
pub const U2F_CMD_AUTH: i32 = 2;
pub const U2F_AUTH_SIGN: i32 = 3;
pub const U2F_AUTH_CHECK: i32 = 7;
pub const CTAP_CID_BROADCAST: i64 = 4294967295;
pub const CTAP_INIT_HEADER_LEN: i32 = 7;
pub const CTAP_CONT_HEADER_LEN: i32 = 5;
pub const CTAP_MAX_REPORT_LEN: i32 = 64;
pub const CTAP_MIN_REPORT_LEN: i32 = 8;
pub const FIDO_RANDOM_DEV: &[u8; 13usize] = b"/dev/urandom\0";
pub const FIDO_MAXMSG: i32 = 2048;
pub const FIDO_CAP_WINK: i32 = 1;
pub const FIDO_CAP_CBOR: i32 = 4;
pub const FIDO_CAP_NMSG: i32 = 8;
pub const COSE_UNSPEC: i32 = 0;
pub const COSE_ES256: i32 = -7;
pub const COSE_EDDSA: i32 = -8;
pub const COSE_ECDH_ES256: i32 = -25;
pub const COSE_ES384: i32 = -35;
pub const COSE_RS256: i32 = -257;
pub const COSE_RS1: i32 = -65535;
pub const COSE_KTY_OKP: i32 = 1;
pub const COSE_KTY_EC2: i32 = 2;
pub const COSE_KTY_RSA: i32 = 3;
pub const COSE_P256: i32 = 1;
pub const COSE_P384: i32 = 2;
pub const COSE_ED25519: i32 = 6;
pub const FIDO_EXT_HMAC_SECRET: i32 = 1;
pub const FIDO_EXT_CRED_PROTECT: i32 = 2;
pub const FIDO_EXT_LARGEBLOB_KEY: i32 = 4;
pub const FIDO_EXT_CRED_BLOB: i32 = 8;
pub const FIDO_EXT_MINPINLEN: i32 = 16;
pub const FIDO_CRED_PROT_UV_OPTIONAL: i32 = 1;
pub const FIDO_CRED_PROT_UV_OPTIONAL_WITH_ID: i32 = 2;
pub const FIDO_CRED_PROT_UV_REQUIRED: i32 = 3;
pub const FIDO_UV_MODE_TUP: i32 = 1;
pub const FIDO_UV_MODE_FP: i32 = 2;
pub const FIDO_UV_MODE_PIN: i32 = 4;
pub const FIDO_UV_MODE_VOICE: i32 = 8;
pub const FIDO_UV_MODE_FACE: i32 = 16;
pub const FIDO_UV_MODE_LOCATION: i32 = 32;
pub const FIDO_UV_MODE_EYE: i32 = 64;
pub const FIDO_UV_MODE_DRAWN: i32 = 128;
pub const FIDO_UV_MODE_HAND: i32 = 256;
pub const FIDO_UV_MODE_NONE: i32 = 512;
pub const FIDO_UV_MODE_ALL: i32 = 1024;
pub const FIDO_UV_MODE_EXT_PIN: i32 = 2048;
pub const FIDO_UV_MODE_EXT_DRAWN: i32 = 4096;
pub const FIDO_DEBUG: i32 = 1;
pub const FIDO_DISABLE_U2F_FALLBACK: i32 = 2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_dev {
    _unused: [u8; 0],
}
pub type fido_dev_io_open_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type fido_dev_io_close_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
pub type fido_dev_io_read_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type fido_dev_io_write_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int,
>;
pub type fido_dev_rx_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut fido_dev,
        arg2: u8,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: usize,
        arg5: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type fido_dev_tx_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut fido_dev,
        arg2: u8,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: usize,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_dev_io {
    pub open: fido_dev_io_open_t,
    pub close: fido_dev_io_close_t,
    pub read: fido_dev_io_read_t,
    pub write: fido_dev_io_write_t,
}
#[test]
fn bindgen_test_layout_fido_dev_io() {
    const UNINIT: ::std::mem::MaybeUninit<fido_dev_io> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fido_dev_io>(),
        32usize,
        concat!("Size of: ", stringify!(fido_dev_io))
    );
    assert_eq!(
        ::std::mem::align_of::<fido_dev_io>(),
        8usize,
        concat!("Alignment of ", stringify!(fido_dev_io))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).open) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_io),
            "::",
            stringify!(open)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).close) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_io),
            "::",
            stringify!(close)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).read) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_io),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).write) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_io),
            "::",
            stringify!(write)
        )
    );
}
pub type fido_dev_io_t = fido_dev_io;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_dev_transport {
    pub rx: fido_dev_rx_t,
    pub tx: fido_dev_tx_t,
}
#[test]
fn bindgen_test_layout_fido_dev_transport() {
    const UNINIT: ::std::mem::MaybeUninit<fido_dev_transport> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fido_dev_transport>(),
        16usize,
        concat!("Size of: ", stringify!(fido_dev_transport))
    );
    assert_eq!(
        ::std::mem::align_of::<fido_dev_transport>(),
        8usize,
        concat!("Alignment of ", stringify!(fido_dev_transport))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rx) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_transport),
            "::",
            stringify!(rx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fido_dev_transport),
            "::",
            stringify!(tx)
        )
    );
}
pub type fido_dev_transport_t = fido_dev_transport;
pub const fido_opt_t_FIDO_OPT_OMIT: fido_opt_t = 0;
pub const fido_opt_t_FIDO_OPT_FALSE: fido_opt_t = 1;
pub const fido_opt_t_FIDO_OPT_TRUE: fido_opt_t = 2;
pub type fido_opt_t = ::std::os::raw::c_int;
pub type fido_log_handler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>;
pub type fido_sigset_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_assert {
    _unused: [u8; 0],
}
pub type fido_assert_t = fido_assert;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_cbor_info {
    _unused: [u8; 0],
}
pub type fido_cbor_info_t = fido_cbor_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_cred {
    _unused: [u8; 0],
}
pub type fido_cred_t = fido_cred;
pub type fido_dev_t = fido_dev;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fido_dev_info {
    _unused: [u8; 0],
}
pub type fido_dev_info_t = fido_dev_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct es256_pk {
    _unused: [u8; 0],
}
pub type es256_pk_t = es256_pk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rs256_pk {
    _unused: [u8; 0],
}
pub type rs256_pk_t = rs256_pk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct eddsa_pk {
    _unused: [u8; 0],
}
pub type eddsa_pk_t = eddsa_pk;
extern "C" {
    pub fn fido_strerr(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn fido_assert_new() -> *mut fido_assert_t;
    pub fn fido_cred_new() -> *mut fido_cred_t;
    pub fn fido_dev_new() -> *mut fido_dev_t;
    pub fn fido_dev_new_with_info(arg1: *const fido_dev_info_t) -> *mut fido_dev_t;
    pub fn fido_dev_info_new(arg1: usize) -> *mut fido_dev_info_t;
    pub fn fido_cbor_info_new() -> *mut fido_cbor_info_t;
    pub fn fido_dev_io_handle(arg1: *const fido_dev_t) -> *mut ::std::os::raw::c_void;
    pub fn fido_assert_free(arg1: *mut *mut fido_assert_t);
    pub fn fido_cbor_info_free(arg1: *mut *mut fido_cbor_info_t);
    pub fn fido_cred_free(arg1: *mut *mut fido_cred_t);
    pub fn fido_dev_force_fido2(arg1: *mut fido_dev_t);
    pub fn fido_dev_force_u2f(arg1: *mut fido_dev_t);
    pub fn fido_dev_free(arg1: *mut *mut fido_dev_t);
    pub fn fido_dev_info_free(arg1: *mut *mut fido_dev_info_t, arg2: usize);
    pub fn fido_init(arg1: ::std::os::raw::c_int);
    pub fn fido_set_log_handler(arg1: fido_log_handler_t);
    pub fn fido_assert_authdata_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_clientdata_hash_ptr(
        arg1: *const fido_assert_t,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_hmac_secret_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_id_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_largeblob_key_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_sig_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_user_id_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_blob_ptr(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cbor_info_certs_name_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn fido_cbor_info_extensions_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn fido_cbor_info_options_name_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn fido_cbor_info_transports_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn fido_cbor_info_versions_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *mut *mut ::std::os::raw::c_char;
    pub fn fido_cbor_info_options_value_ptr(arg1: *const fido_cbor_info_t) -> *const bool;
    pub fn fido_assert_rp_id(arg1: *const fido_assert_t) -> *const ::std::os::raw::c_char;
    pub fn fido_assert_user_display_name(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_assert_user_icon(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_assert_user_name(
        arg1: *const fido_assert_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_cbor_info_algorithm_type(
        arg1: *const fido_cbor_info_t,
        arg2: usize,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_cred_display_name(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_char;
    pub fn fido_cred_fmt(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_char;
    pub fn fido_cred_rp_id(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_char;
    pub fn fido_cred_rp_name(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_char;
    pub fn fido_cred_user_name(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_char;
    pub fn fido_dev_info_manufacturer_string(
        arg1: *const fido_dev_info_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_dev_info_path(arg1: *const fido_dev_info_t) -> *const ::std::os::raw::c_char;
    pub fn fido_dev_info_product_string(
        arg1: *const fido_dev_info_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn fido_dev_info_ptr(arg1: *const fido_dev_info_t, arg2: usize) -> *const fido_dev_info_t;
    pub fn fido_cbor_info_protocols_ptr(arg1: *const fido_cbor_info_t) -> *const u8;
    pub fn fido_cbor_info_certs_value_ptr(arg1: *const fido_cbor_info_t) -> *const u64;
    pub fn fido_cbor_info_aaguid_ptr(
        arg1: *const fido_cbor_info_t,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_aaguid_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_attstmt_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_authdata_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_authdata_raw_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_clientdata_hash_ptr(
        arg1: *const fido_cred_t,
    ) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_id_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_largeblob_key_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_pubkey_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_sig_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_user_id_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_cred_x5c_ptr(arg1: *const fido_cred_t) -> *const ::std::os::raw::c_uchar;
    pub fn fido_assert_allow_cred(
        arg1: *mut fido_assert_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_authdata(
        arg1: *mut fido_assert_t,
        arg2: usize,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_authdata_raw(
        arg1: *mut fido_assert_t,
        arg2: usize,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_clientdata(
        arg1: *mut fido_assert_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_clientdata_hash(
        arg1: *mut fido_assert_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_count(arg1: *mut fido_assert_t, arg2: usize) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_extensions(
        arg1: *mut fido_assert_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_hmac_salt(
        arg1: *mut fido_assert_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_hmac_secret(
        arg1: *mut fido_assert_t,
        arg2: usize,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_options(
        arg1: *mut fido_assert_t,
        arg2: bool,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_rp(
        arg1: *mut fido_assert_t,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_up(arg1: *mut fido_assert_t, arg2: fido_opt_t) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_uv(arg1: *mut fido_assert_t, arg2: fido_opt_t) -> ::std::os::raw::c_int;
    pub fn fido_assert_set_sig(
        arg1: *mut fido_assert_t,
        arg2: usize,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_verify(
        arg1: *const fido_assert_t,
        arg2: usize,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cbor_info_algorithm_cose(
        arg1: *const fido_cbor_info_t,
        arg2: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_exclude(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_prot(arg1: *const fido_cred_t) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_attstmt(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_authdata(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_authdata_raw(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_blob(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_clientdata(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_clientdata_hash(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_extensions(
        arg1: *mut fido_cred_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_fmt(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_id(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_options(
        arg1: *mut fido_cred_t,
        arg2: bool,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_pin_minlen(arg1: *mut fido_cred_t, arg2: usize) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_prot(
        arg1: *mut fido_cred_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_rk(arg1: *mut fido_cred_t, arg2: fido_opt_t) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_rp(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_sig(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_type(
        arg1: *mut fido_cred_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_uv(arg1: *mut fido_cred_t, arg2: fido_opt_t) -> ::std::os::raw::c_int;
    pub fn fido_cred_type(arg1: *const fido_cred_t) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_user(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: *const ::std::os::raw::c_char,
        arg5: *const ::std::os::raw::c_char,
        arg6: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_set_x509(
        arg1: *mut fido_cred_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_cred_verify(arg1: *const fido_cred_t) -> ::std::os::raw::c_int;
    pub fn fido_cred_verify_self(arg1: *const fido_cred_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_set_sigmask(
        arg1: *mut fido_dev_t,
        arg2: *const fido_sigset_t,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_cancel(arg1: *mut fido_dev_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_close(arg1: *mut fido_dev_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_assert(
        arg1: *mut fido_dev_t,
        arg2: *mut fido_assert_t,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_cbor_info(
        arg1: *mut fido_dev_t,
        arg2: *mut fido_cbor_info_t,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_retry_count(
        arg1: *mut fido_dev_t,
        arg2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_uv_retry_count(
        arg1: *mut fido_dev_t,
        arg2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_touch_begin(arg1: *mut fido_dev_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_get_touch_status(
        arg1: *mut fido_dev_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_info_manifest(
        arg1: *mut fido_dev_info_t,
        arg2: usize,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_info_set(
        arg1: *mut fido_dev_info_t,
        arg2: usize,
        arg3: *const ::std::os::raw::c_char,
        arg4: *const ::std::os::raw::c_char,
        arg5: *const ::std::os::raw::c_char,
        arg6: *const fido_dev_io_t,
        arg7: *const fido_dev_transport_t,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_make_cred(
        arg1: *mut fido_dev_t,
        arg2: *mut fido_cred_t,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_open_with_info(arg1: *mut fido_dev_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_open(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_reset(arg1: *mut fido_dev_t) -> ::std::os::raw::c_int;
    pub fn fido_dev_set_io_functions(
        arg1: *mut fido_dev_t,
        arg2: *const fido_dev_io_t,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_set_pin(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_set_transport_functions(
        arg1: *mut fido_dev_t,
        arg2: *const fido_dev_transport_t,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_set_timeout(
        arg1: *mut fido_dev_t,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn fido_assert_authdata_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_clientdata_hash_len(arg1: *const fido_assert_t) -> usize;
    pub fn fido_assert_count(arg1: *const fido_assert_t) -> usize;
    pub fn fido_assert_hmac_secret_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_id_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_largeblob_key_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_sig_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_user_id_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_assert_blob_len(arg1: *const fido_assert_t, arg2: usize) -> usize;
    pub fn fido_cbor_info_aaguid_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_algorithm_count(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_certs_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_extensions_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_options_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_protocols_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_transports_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cbor_info_versions_len(arg1: *const fido_cbor_info_t) -> usize;
    pub fn fido_cred_aaguid_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_attstmt_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_authdata_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_authdata_raw_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_clientdata_hash_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_id_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_largeblob_key_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_pin_minlen(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_pubkey_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_sig_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_user_id_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_cred_x5c_len(arg1: *const fido_cred_t) -> usize;
    pub fn fido_assert_flags(arg1: *const fido_assert_t, arg2: usize) -> u8;
    pub fn fido_assert_sigcount(arg1: *const fido_assert_t, arg2: usize) -> u32;
    pub fn fido_cred_flags(arg1: *const fido_cred_t) -> u8;
    pub fn fido_cred_sigcount(arg1: *const fido_cred_t) -> u32;
    pub fn fido_dev_protocol(arg1: *const fido_dev_t) -> u8;
    pub fn fido_dev_major(arg1: *const fido_dev_t) -> u8;
    pub fn fido_dev_minor(arg1: *const fido_dev_t) -> u8;
    pub fn fido_dev_build(arg1: *const fido_dev_t) -> u8;
    pub fn fido_dev_flags(arg1: *const fido_dev_t) -> u8;
    pub fn fido_dev_info_vendor(arg1: *const fido_dev_info_t) -> i16;
    pub fn fido_dev_info_product(arg1: *const fido_dev_info_t) -> i16;
    pub fn fido_cbor_info_fwversion(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxcredbloblen(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxcredcntlst(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxcredidlen(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxlargeblob(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxmsgsiz(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_maxrpid_minpinlen(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_minpinlen(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_uv_attempts(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_uv_modality(arg1: *const fido_cbor_info_t) -> u64;
    pub fn fido_cbor_info_rk_remaining(arg1: *const fido_cbor_info_t) -> i64;
    pub fn fido_dev_has_pin(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_has_uv(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_is_fido2(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_is_winhello(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_supports_credman(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_supports_cred_prot(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_supports_permissions(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_supports_pin(arg1: *const fido_dev_t) -> bool;
    pub fn fido_dev_supports_uv(arg1: *const fido_dev_t) -> bool;
    pub fn fido_cbor_info_new_pin_required(arg1: *const fido_cbor_info_t) -> bool;
    pub fn fido_dev_largeblob_get(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: *mut *mut ::std::os::raw::c_uchar,
        arg5: *mut usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_largeblob_set(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: *const ::std::os::raw::c_uchar,
        arg5: usize,
        arg6: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_largeblob_remove(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_largeblob_get_array(
        arg1: *mut fido_dev_t,
        arg2: *mut *mut ::std::os::raw::c_uchar,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
    pub fn fido_dev_largeblob_set_array(
        arg1: *mut fido_dev_t,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: usize,
        arg4: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
