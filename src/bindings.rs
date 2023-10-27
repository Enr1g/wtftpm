#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
pub const PKCS11_H: u32 = 1;
pub const CRYPTOKI_VERSION_MAJOR: u32 = 2;
pub const CRYPTOKI_VERSION_MINOR: u32 = 40;
pub const P11_KIT_CRYPTOKI_VERSION_REVISION: u32 = 0;
pub const CRYPTOKI_COMPAT: u32 = 1;
pub const CKN_SURRENDER: u32 = 0;
pub const CKF_TOKEN_PRESENT: u32 = 1;
pub const CKF_REMOVABLE_DEVICE: u32 = 2;
pub const CKF_HW_SLOT: u32 = 4;
pub const CKF_ARRAY_ATTRIBUTE: u32 = 1073741824;
pub const CKF_RNG: u32 = 1;
pub const CKF_WRITE_PROTECTED: u32 = 2;
pub const CKF_LOGIN_REQUIRED: u32 = 4;
pub const CKF_USER_PIN_INITIALIZED: u32 = 8;
pub const CKF_RESTORE_KEY_NOT_NEEDED: u32 = 32;
pub const CKF_CLOCK_ON_TOKEN: u32 = 64;
pub const CKF_PROTECTED_AUTHENTICATION_PATH: u32 = 256;
pub const CKF_DUAL_CRYPTO_OPERATIONS: u32 = 512;
pub const CKF_TOKEN_INITIALIZED: u32 = 1024;
pub const CKF_SECONDARY_AUTHENTICATION: u32 = 2048;
pub const CKF_USER_PIN_COUNT_LOW: u32 = 65536;
pub const CKF_USER_PIN_FINAL_TRY: u32 = 131072;
pub const CKF_USER_PIN_LOCKED: u32 = 262144;
pub const CKF_USER_PIN_TO_BE_CHANGED: u32 = 524288;
pub const CKF_SO_PIN_COUNT_LOW: u32 = 1048576;
pub const CKF_SO_PIN_FINAL_TRY: u32 = 2097152;
pub const CKF_SO_PIN_LOCKED: u32 = 4194304;
pub const CKF_SO_PIN_TO_BE_CHANGED: u32 = 8388608;
pub const CK_EFFECTIVELY_INFINITE: u32 = 0;
pub const CK_INVALID_HANDLE: u32 = 0;
pub const CKU_SO: u32 = 0;
pub const CKU_USER: u32 = 1;
pub const CKU_CONTEXT_SPECIFIC: u32 = 2;
pub const CKS_RO_PUBLIC_SESSION: u32 = 0;
pub const CKS_RO_USER_FUNCTIONS: u32 = 1;
pub const CKS_RW_PUBLIC_SESSION: u32 = 2;
pub const CKS_RW_USER_FUNCTIONS: u32 = 3;
pub const CKS_RW_SO_FUNCTIONS: u32 = 4;
pub const CKF_RW_SESSION: u32 = 2;
pub const CKF_SERIAL_SESSION: u32 = 4;
pub const CKO_DATA: u32 = 0;
pub const CKO_CERTIFICATE: u32 = 1;
pub const CKO_PUBLIC_KEY: u32 = 2;
pub const CKO_PRIVATE_KEY: u32 = 3;
pub const CKO_SECRET_KEY: u32 = 4;
pub const CKO_HW_FEATURE: u32 = 5;
pub const CKO_DOMAIN_PARAMETERS: u32 = 6;
pub const CKO_MECHANISM: u32 = 7;
pub const CKO_OTP_KEY: u32 = 8;
pub const CKH_MONOTONIC_COUNTER: u32 = 1;
pub const CKH_CLOCK: u32 = 2;
pub const CKH_USER_INTERFACE: u32 = 3;
pub const CKK_RSA: u32 = 0;
pub const CKK_DSA: u32 = 1;
pub const CKK_DH: u32 = 2;
pub const CKK_ECDSA: u32 = 3;
pub const CKK_EC: u32 = 3;
pub const CKK_X9_42_DH: u32 = 4;
pub const CKK_KEA: u32 = 5;
pub const CKK_GENERIC_SECRET: u32 = 16;
pub const CKK_RC2: u32 = 17;
pub const CKK_RC4: u32 = 18;
pub const CKK_DES: u32 = 19;
pub const CKK_DES2: u32 = 20;
pub const CKK_DES3: u32 = 21;
pub const CKK_CAST: u32 = 22;
pub const CKK_CAST3: u32 = 23;
pub const CKK_CAST128: u32 = 24;
pub const CKK_RC5: u32 = 25;
pub const CKK_IDEA: u32 = 26;
pub const CKK_SKIPJACK: u32 = 27;
pub const CKK_BATON: u32 = 28;
pub const CKK_JUNIPER: u32 = 29;
pub const CKK_CDMF: u32 = 30;
pub const CKK_AES: u32 = 31;
pub const CKK_BLOWFISH: u32 = 32;
pub const CKK_TWOFISH: u32 = 33;
pub const CKK_SECURID: u32 = 34;
pub const CKK_HOTP: u32 = 35;
pub const CKK_ACTI: u32 = 36;
pub const CKK_CAMELLIA: u32 = 37;
pub const CKK_ARIA: u32 = 38;
pub const CKK_MD5_HMAC: u32 = 39;
pub const CKK_SHA_1_HMAC: u32 = 40;
pub const CKK_RIPEMD128_HMAC: u32 = 41;
pub const CKK_RIPEMD160_HMAC: u32 = 42;
pub const CKK_SHA256_HMAC: u32 = 43;
pub const CKK_SHA384_HMAC: u32 = 44;
pub const CKK_SHA512_HMAC: u32 = 45;
pub const CKK_SHA224_HMAC: u32 = 46;
pub const CKK_SEED: u32 = 47;
pub const CKK_GOSTR3410: u32 = 48;
pub const CKK_GOSTR3411: u32 = 49;
pub const CKK_GOST28147: u32 = 50;
pub const CKK_EC_EDWARDS: u32 = 64;
pub const CKC_X_509: u32 = 0;
pub const CKC_X_509_ATTR_CERT: u32 = 1;
pub const CKC_WTLS: u32 = 2;
pub const CKA_CLASS: u32 = 0;
pub const CKA_TOKEN: u32 = 1;
pub const CKA_PRIVATE: u32 = 2;
pub const CKA_LABEL: u32 = 3;
pub const CKA_APPLICATION: u32 = 16;
pub const CKA_VALUE: u32 = 17;
pub const CKA_OBJECT_ID: u32 = 18;
pub const CKA_CERTIFICATE_TYPE: u32 = 128;
pub const CKA_ISSUER: u32 = 129;
pub const CKA_SERIAL_NUMBER: u32 = 130;
pub const CKA_AC_ISSUER: u32 = 131;
pub const CKA_OWNER: u32 = 132;
pub const CKA_ATTR_TYPES: u32 = 133;
pub const CKA_TRUSTED: u32 = 134;
pub const CKA_CERTIFICATE_CATEGORY: u32 = 135;
pub const CKA_JAVA_MIDP_SECURITY_DOMAIN: u32 = 136;
pub const CKA_URL: u32 = 137;
pub const CKA_HASH_OF_SUBJECT_PUBLIC_KEY: u32 = 138;
pub const CKA_HASH_OF_ISSUER_PUBLIC_KEY: u32 = 139;
pub const CKA_NAME_HASH_ALGORITHM: u32 = 140;
pub const CKA_CHECK_VALUE: u32 = 144;
pub const CKA_KEY_TYPE: u32 = 256;
pub const CKA_SUBJECT: u32 = 257;
pub const CKA_ID: u32 = 258;
pub const CKA_SENSITIVE: u32 = 259;
pub const CKA_ENCRYPT: u32 = 260;
pub const CKA_DECRYPT: u32 = 261;
pub const CKA_WRAP: u32 = 262;
pub const CKA_UNWRAP: u32 = 263;
pub const CKA_SIGN: u32 = 264;
pub const CKA_SIGN_RECOVER: u32 = 265;
pub const CKA_VERIFY: u32 = 266;
pub const CKA_VERIFY_RECOVER: u32 = 267;
pub const CKA_DERIVE: u32 = 268;
pub const CKA_START_DATE: u32 = 272;
pub const CKA_END_DATE: u32 = 273;
pub const CKA_MODULUS: u32 = 288;
pub const CKA_MODULUS_BITS: u32 = 289;
pub const CKA_PUBLIC_EXPONENT: u32 = 290;
pub const CKA_PRIVATE_EXPONENT: u32 = 291;
pub const CKA_PRIME_1: u32 = 292;
pub const CKA_PRIME_2: u32 = 293;
pub const CKA_EXPONENT_1: u32 = 294;
pub const CKA_EXPONENT_2: u32 = 295;
pub const CKA_COEFFICIENT: u32 = 296;
pub const CKA_PUBLIC_KEY_INFO: u32 = 297;
pub const CKA_PRIME: u32 = 304;
pub const CKA_SUBPRIME: u32 = 305;
pub const CKA_BASE: u32 = 306;
pub const CKA_PRIME_BITS: u32 = 307;
pub const CKA_SUB_PRIME_BITS: u32 = 308;
pub const CKA_VALUE_BITS: u32 = 352;
pub const CKA_VALUE_LEN: u32 = 353;
pub const CKA_EXTRACTABLE: u32 = 354;
pub const CKA_LOCAL: u32 = 355;
pub const CKA_NEVER_EXTRACTABLE: u32 = 356;
pub const CKA_ALWAYS_SENSITIVE: u32 = 357;
pub const CKA_KEY_GEN_MECHANISM: u32 = 358;
pub const CKA_MODIFIABLE: u32 = 368;
pub const CKA_COPYABLE: u32 = 369;
pub const CKA_DESTROYABLE: u32 = 370;
pub const CKA_ECDSA_PARAMS: u32 = 384;
pub const CKA_EC_PARAMS: u32 = 384;
pub const CKA_EC_POINT: u32 = 385;
pub const CKA_SECONDARY_AUTH: u32 = 512;
pub const CKA_AUTH_PIN_FLAGS: u32 = 513;
pub const CKA_ALWAYS_AUTHENTICATE: u32 = 514;
pub const CKA_WRAP_WITH_TRUSTED: u32 = 528;
pub const CKA_OTP_FORMAT: u32 = 544;
pub const CKA_OTP_LENGTH: u32 = 545;
pub const CKA_OTP_TIME_INTERVAL: u32 = 546;
pub const CKA_OTP_USER_FRIENDLY_MODE: u32 = 547;
pub const CKA_OTP_CHALLENGE_REQUIREMENT: u32 = 548;
pub const CKA_OTP_TIME_REQUIREMENT: u32 = 549;
pub const CKA_OTP_COUNTER_REQUIREMENT: u32 = 550;
pub const CKA_OTP_PIN_REQUIREMENT: u32 = 551;
pub const CKA_OTP_USER_IDENTIFIER: u32 = 554;
pub const CKA_OTP_SERVICE_IDENTIFIER: u32 = 555;
pub const CKA_OTP_SERVICE_LOGO: u32 = 556;
pub const CKA_OTP_SERVICE_LOGO_TYPE: u32 = 557;
pub const CKA_OTP_COUNTER: u32 = 558;
pub const CKA_OTP_TIME: u32 = 559;
pub const CKA_GOSTR3410_PARAMS: u32 = 592;
pub const CKA_GOSTR3411_PARAMS: u32 = 593;
pub const CKA_GOST28147_PARAMS: u32 = 594;
pub const CKA_HW_FEATURE_TYPE: u32 = 768;
pub const CKA_RESET_ON_INIT: u32 = 769;
pub const CKA_HAS_RESET: u32 = 770;
pub const CKA_PIXEL_X: u32 = 1024;
pub const CKA_PIXEL_Y: u32 = 1025;
pub const CKA_RESOLUTION: u32 = 1026;
pub const CKA_CHAR_ROWS: u32 = 1027;
pub const CKA_CHAR_COLUMNS: u32 = 1028;
pub const CKA_COLOR: u32 = 1029;
pub const CKA_BITS_PER_PIXEL: u32 = 1030;
pub const CKA_CHAR_SETS: u32 = 1152;
pub const CKA_ENCODING_METHODS: u32 = 1153;
pub const CKA_MIME_TYPES: u32 = 1154;
pub const CKA_MECHANISM_TYPE: u32 = 1280;
pub const CKA_REQUIRED_CMS_ATTRIBUTES: u32 = 1281;
pub const CKA_DEFAULT_CMS_ATTRIBUTES: u32 = 1282;
pub const CKA_SUPPORTED_CMS_ATTRIBUTES: u32 = 1283;
pub const CKA_WRAP_TEMPLATE: u32 = 1073742353;
pub const CKA_UNWRAP_TEMPLATE: u32 = 1073742354;
pub const CKA_DERIVE_TEMPLATE: u32 = 1073742355;
pub const CKA_ALLOWED_MECHANISMS: u32 = 1073743360;
pub const CKM_RSA_PKCS_KEY_PAIR_GEN: u32 = 0;
pub const CKM_RSA_PKCS: u32 = 1;
pub const CKM_RSA_9796: u32 = 2;
pub const CKM_RSA_X_509: u32 = 3;
pub const CKM_MD2_RSA_PKCS: u32 = 4;
pub const CKM_MD5_RSA_PKCS: u32 = 5;
pub const CKM_SHA1_RSA_PKCS: u32 = 6;
pub const CKM_RIPEMD128_RSA_PKCS: u32 = 7;
pub const CKM_RIPEMD160_RSA_PKCS: u32 = 8;
pub const CKM_RSA_PKCS_OAEP: u32 = 9;
pub const CKM_RSA_X9_31_KEY_PAIR_GEN: u32 = 10;
pub const CKM_RSA_X9_31: u32 = 11;
pub const CKM_SHA1_RSA_X9_31: u32 = 12;
pub const CKM_RSA_PKCS_PSS: u32 = 13;
pub const CKM_SHA1_RSA_PKCS_PSS: u32 = 14;
pub const CKM_DSA_KEY_PAIR_GEN: u32 = 16;
pub const CKM_DSA: u32 = 17;
pub const CKM_DSA_SHA1: u32 = 18;
pub const CKM_DSA_SHA224: u32 = 19;
pub const CKM_DSA_SHA256: u32 = 20;
pub const CKM_DSA_SHA384: u32 = 21;
pub const CKM_DSA_SHA512: u32 = 22;
pub const CKM_DH_PKCS_KEY_PAIR_GEN: u32 = 32;
pub const CKM_DH_PKCS_DERIVE: u32 = 33;
pub const CKM_X9_42_DH_KEY_PAIR_GEN: u32 = 48;
pub const CKM_X9_42_DH_DERIVE: u32 = 49;
pub const CKM_X9_42_DH_HYBRID_DERIVE: u32 = 50;
pub const CKM_X9_42_MQV_DERIVE: u32 = 51;
pub const CKM_SHA256_RSA_PKCS: u32 = 64;
pub const CKM_SHA384_RSA_PKCS: u32 = 65;
pub const CKM_SHA512_RSA_PKCS: u32 = 66;
pub const CKM_SHA256_RSA_PKCS_PSS: u32 = 67;
pub const CKM_SHA384_RSA_PKCS_PSS: u32 = 68;
pub const CKM_SHA512_RSA_PKCS_PSS: u32 = 69;
pub const CKM_SHA512_224: u32 = 72;
pub const CKM_SHA512_224_HMAC: u32 = 73;
pub const CKM_SHA512_224_HMAC_GENERAL: u32 = 74;
pub const CKM_SHA512_224_KEY_DERIVATION: u32 = 75;
pub const CKM_SHA512_256: u32 = 76;
pub const CKM_SHA512_256_HMAC: u32 = 77;
pub const CKM_SHA512_256_HMAC_GENERAL: u32 = 78;
pub const CKM_SHA512_256_KEY_DERIVATION: u32 = 79;
pub const CKM_SHA512_T: u32 = 80;
pub const CKM_SHA512_T_HMAC: u32 = 81;
pub const CKM_SHA512_T_HMAC_GENERAL: u32 = 82;
pub const CKM_SHA512_T_KEY_DERIVATION: u32 = 83;
pub const CKM_RC2_KEY_GEN: u32 = 256;
pub const CKM_RC2_ECB: u32 = 257;
pub const CKM_RC2_CBC: u32 = 258;
pub const CKM_RC2_MAC: u32 = 259;
pub const CKM_RC2_MAC_GENERAL: u32 = 260;
pub const CKM_RC2_CBC_PAD: u32 = 261;
pub const CKM_RC4_KEY_GEN: u32 = 272;
pub const CKM_RC4: u32 = 273;
pub const CKM_DES_KEY_GEN: u32 = 288;
pub const CKM_DES_ECB: u32 = 289;
pub const CKM_DES_CBC: u32 = 290;
pub const CKM_DES_MAC: u32 = 291;
pub const CKM_DES_MAC_GENERAL: u32 = 292;
pub const CKM_DES_CBC_PAD: u32 = 293;
pub const CKM_DES2_KEY_GEN: u32 = 304;
pub const CKM_DES3_KEY_GEN: u32 = 305;
pub const CKM_DES3_ECB: u32 = 306;
pub const CKM_DES3_CBC: u32 = 307;
pub const CKM_DES3_MAC: u32 = 308;
pub const CKM_DES3_MAC_GENERAL: u32 = 309;
pub const CKM_DES3_CBC_PAD: u32 = 310;
pub const CKM_DES3_CMAC_GENERAL: u32 = 311;
pub const CKM_DES3_CMAC: u32 = 312;
pub const CKM_CDMF_KEY_GEN: u32 = 320;
pub const CKM_CDMF_ECB: u32 = 321;
pub const CKM_CDMF_CBC: u32 = 322;
pub const CKM_CDMF_MAC: u32 = 323;
pub const CKM_CDMF_MAC_GENERAL: u32 = 324;
pub const CKM_CDMF_CBC_PAD: u32 = 325;
pub const CKM_DES_OFB64: u32 = 336;
pub const CKM_DES_OFB8: u32 = 337;
pub const CKM_DES_CFB64: u32 = 338;
pub const CKM_DES_CFB8: u32 = 339;
pub const CKM_MD2: u32 = 512;
pub const CKM_MD2_HMAC: u32 = 513;
pub const CKM_MD2_HMAC_GENERAL: u32 = 514;
pub const CKM_MD5: u32 = 528;
pub const CKM_MD5_HMAC: u32 = 529;
pub const CKM_MD5_HMAC_GENERAL: u32 = 530;
pub const CKM_SHA_1: u32 = 544;
pub const CKM_SHA_1_HMAC: u32 = 545;
pub const CKM_SHA_1_HMAC_GENERAL: u32 = 546;
pub const CKM_RIPEMD128: u32 = 560;
pub const CKM_RIPEMD128_HMAC: u32 = 561;
pub const CKM_RIPEMD128_HMAC_GENERAL: u32 = 562;
pub const CKM_RIPEMD160: u32 = 576;
pub const CKM_RIPEMD160_HMAC: u32 = 577;
pub const CKM_RIPEMD160_HMAC_GENERAL: u32 = 578;
pub const CKM_SHA256: u32 = 592;
pub const CKM_SHA256_HMAC: u32 = 593;
pub const CKM_SHA256_HMAC_GENERAL: u32 = 594;
pub const CKM_SHA384: u32 = 608;
pub const CKM_SHA384_HMAC: u32 = 609;
pub const CKM_SHA384_HMAC_GENERAL: u32 = 610;
pub const CKM_SHA512: u32 = 624;
pub const CKM_SHA512_HMAC: u32 = 625;
pub const CKM_SHA512_HMAC_GENERAL: u32 = 626;
pub const CKM_SECURID_KEY_GEN: u32 = 640;
pub const CKM_SECURID: u32 = 642;
pub const CKM_HOTP_KEY_GEN: u32 = 656;
pub const CKM_HOTP: u32 = 657;
pub const CKM_ACTI: u32 = 672;
pub const CKM_ACTI_KEY_GEN: u32 = 673;
pub const CKM_CAST_KEY_GEN: u32 = 768;
pub const CKM_CAST_ECB: u32 = 769;
pub const CKM_CAST_CBC: u32 = 770;
pub const CKM_CAST_MAC: u32 = 771;
pub const CKM_CAST_MAC_GENERAL: u32 = 772;
pub const CKM_CAST_CBC_PAD: u32 = 773;
pub const CKM_CAST3_KEY_GEN: u32 = 784;
pub const CKM_CAST3_ECB: u32 = 785;
pub const CKM_CAST3_CBC: u32 = 786;
pub const CKM_CAST3_MAC: u32 = 787;
pub const CKM_CAST3_MAC_GENERAL: u32 = 788;
pub const CKM_CAST3_CBC_PAD: u32 = 789;
pub const CKM_CAST5_KEY_GEN: u32 = 800;
pub const CKM_CAST128_KEY_GEN: u32 = 800;
pub const CKM_CAST5_ECB: u32 = 801;
pub const CKM_CAST128_ECB: u32 = 801;
pub const CKM_CAST5_CBC: u32 = 802;
pub const CKM_CAST128_CBC: u32 = 802;
pub const CKM_CAST5_MAC: u32 = 803;
pub const CKM_CAST128_MAC: u32 = 803;
pub const CKM_CAST5_MAC_GENERAL: u32 = 804;
pub const CKM_CAST128_MAC_GENERAL: u32 = 804;
pub const CKM_CAST5_CBC_PAD: u32 = 805;
pub const CKM_CAST128_CBC_PAD: u32 = 805;
pub const CKM_RC5_KEY_GEN: u32 = 816;
pub const CKM_RC5_ECB: u32 = 817;
pub const CKM_RC5_CBC: u32 = 818;
pub const CKM_RC5_MAC: u32 = 819;
pub const CKM_RC5_MAC_GENERAL: u32 = 820;
pub const CKM_RC5_CBC_PAD: u32 = 821;
pub const CKM_IDEA_KEY_GEN: u32 = 832;
pub const CKM_IDEA_ECB: u32 = 833;
pub const CKM_IDEA_CBC: u32 = 834;
pub const CKM_IDEA_MAC: u32 = 835;
pub const CKM_IDEA_MAC_GENERAL: u32 = 836;
pub const CKM_IDEA_CBC_PAD: u32 = 837;
pub const CKM_GENERIC_SECRET_KEY_GEN: u32 = 848;
pub const CKM_CONCATENATE_BASE_AND_KEY: u32 = 864;
pub const CKM_CONCATENATE_BASE_AND_DATA: u32 = 866;
pub const CKM_CONCATENATE_DATA_AND_BASE: u32 = 867;
pub const CKM_XOR_BASE_AND_DATA: u32 = 868;
pub const CKM_EXTRACT_KEY_FROM_KEY: u32 = 869;
pub const CKM_SSL3_PRE_MASTER_KEY_GEN: u32 = 880;
pub const CKM_SSL3_MASTER_KEY_DERIVE: u32 = 881;
pub const CKM_SSL3_KEY_AND_MAC_DERIVE: u32 = 882;
pub const CKM_SSL3_MASTER_KEY_DERIVE_DH: u32 = 883;
pub const CKM_TLS_PRE_MASTER_KEY_GEN: u32 = 884;
pub const CKM_TLS_MASTER_KEY_DERIVE: u32 = 885;
pub const CKM_TLS_KEY_AND_MAC_DERIVE: u32 = 886;
pub const CKM_TLS_MASTER_KEY_DERIVE_DH: u32 = 887;
pub const CKM_TLS_PRF: u32 = 888;
pub const CKM_SSL3_MD5_MAC: u32 = 896;
pub const CKM_SSL3_SHA1_MAC: u32 = 897;
pub const CKM_MD5_KEY_DERIVATION: u32 = 912;
pub const CKM_MD2_KEY_DERIVATION: u32 = 913;
pub const CKM_SHA1_KEY_DERIVATION: u32 = 914;
pub const CKM_SHA256_KEY_DERIVATION: u32 = 915;
pub const CKM_SHA384_KEY_DERIVATION: u32 = 916;
pub const CKM_SHA512_KEY_DERIVATION: u32 = 917;
pub const CKM_PBE_MD2_DES_CBC: u32 = 928;
pub const CKM_PBE_MD5_DES_CBC: u32 = 929;
pub const CKM_PBE_MD5_CAST_CBC: u32 = 930;
pub const CKM_PBE_MD5_CAST3_CBC: u32 = 931;
pub const CKM_PBE_MD5_CAST5_CBC: u32 = 932;
pub const CKM_PBE_MD5_CAST128_CBC: u32 = 932;
pub const CKM_PBE_SHA1_CAST5_CBC: u32 = 933;
pub const CKM_PBE_SHA1_CAST128_CBC: u32 = 933;
pub const CKM_PBE_SHA1_RC4_128: u32 = 934;
pub const CKM_PBE_SHA1_RC4_40: u32 = 935;
pub const CKM_PBE_SHA1_DES3_EDE_CBC: u32 = 936;
pub const CKM_PBE_SHA1_DES2_EDE_CBC: u32 = 937;
pub const CKM_PBE_SHA1_RC2_128_CBC: u32 = 938;
pub const CKM_PBE_SHA1_RC2_40_CBC: u32 = 939;
pub const CKM_PKCS5_PBKD2: u32 = 944;
pub const CKM_PBA_SHA1_WITH_SHA1_HMAC: u32 = 960;
pub const CKM_WTLS_PRE_MASTER_KEY_GEN: u32 = 976;
pub const CKM_WTLS_MASTER_KEY_DERIVE: u32 = 977;
pub const CKM_WTLS_MASTER_KEY_DERIVE_DH_ECC: u32 = 978;
pub const CKM_WTLS_PRF: u32 = 979;
pub const CKM_WTLS_SERVER_KEY_AND_MAC_DERIVE: u32 = 980;
pub const CKM_WTLS_CLIENT_KEY_AND_MAC_DERIVE: u32 = 981;
pub const CKM_TLS10_MAC_SERVER: u32 = 982;
pub const CKM_TLS10_MAC_CLIENT: u32 = 983;
pub const CKM_TLS12_MAC: u32 = 984;
pub const CKM_TLS12_KDF: u32 = 985;
pub const CKM_TLS12_MASTER_KEY_DERIVE: u32 = 992;
pub const CKM_TLS12_KEY_AND_MAC_DERIVE: u32 = 993;
pub const CKM_TLS12_MASTER_KEY_DERIVE_DH: u32 = 994;
pub const CKM_TLS12_KEY_SAFE_DERIVE: u32 = 995;
pub const CKM_TLS_MAC: u32 = 996;
pub const CKM_TLS_KDF: u32 = 997;
pub const CKM_KEY_WRAP_LYNKS: u32 = 1024;
pub const CKM_KEY_WRAP_SET_OAEP: u32 = 1025;
pub const CKM_CMS_SIG: u32 = 1280;
pub const CKM_KIP_DERIVE: u32 = 1296;
pub const CKM_KIP_WRAP: u32 = 1297;
pub const CKM_KIP_MAC: u32 = 1298;
pub const CKM_ARIA_KEY_GEN: u32 = 1376;
pub const CKM_ARIA_ECB: u32 = 1377;
pub const CKM_ARIA_CBC: u32 = 1378;
pub const CKM_ARIA_MAC: u32 = 1379;
pub const CKM_ARIA_MAC_GENERAL: u32 = 1380;
pub const CKM_ARIA_CBC_PAD: u32 = 1381;
pub const CKM_ARIA_ECB_ENCRYPT_DATA: u32 = 1382;
pub const CKM_ARIA_CBC_ENCRYPT_DATA: u32 = 1383;
pub const CKM_SEED_KEY_GEN: u32 = 1616;
pub const CKM_SEED_ECB: u32 = 1617;
pub const CKM_SEED_CBC: u32 = 1618;
pub const CKM_SEED_MAC: u32 = 1619;
pub const CKM_SEED_MAC_GENERAL: u32 = 1620;
pub const CKM_SEED_CBC_PAD: u32 = 1621;
pub const CKM_SEED_ECB_ENCRYPT_DATA: u32 = 1622;
pub const CKM_SEED_CBC_ENCRYPT_DATA: u32 = 1623;
pub const CKM_SKIPJACK_KEY_GEN: u32 = 4096;
pub const CKM_SKIPJACK_ECB64: u32 = 4097;
pub const CKM_SKIPJACK_CBC64: u32 = 4098;
pub const CKM_SKIPJACK_OFB64: u32 = 4099;
pub const CKM_SKIPJACK_CFB64: u32 = 4100;
pub const CKM_SKIPJACK_CFB32: u32 = 4101;
pub const CKM_SKIPJACK_CFB16: u32 = 4102;
pub const CKM_SKIPJACK_CFB8: u32 = 4103;
pub const CKM_SKIPJACK_WRAP: u32 = 4104;
pub const CKM_SKIPJACK_PRIVATE_WRAP: u32 = 4105;
pub const CKM_SKIPJACK_RELAYX: u32 = 4106;
pub const CKM_KEA_KEY_PAIR_GEN: u32 = 4112;
pub const CKM_KEA_KEY_DERIVE: u32 = 4113;
pub const CKM_FORTEZZA_TIMESTAMP: u32 = 4128;
pub const CKM_BATON_KEY_GEN: u32 = 4144;
pub const CKM_BATON_ECB128: u32 = 4145;
pub const CKM_BATON_ECB96: u32 = 4146;
pub const CKM_BATON_CBC128: u32 = 4147;
pub const CKM_BATON_COUNTER: u32 = 4148;
pub const CKM_BATON_SHUFFLE: u32 = 4149;
pub const CKM_BATON_WRAP: u32 = 4150;
pub const CKM_ECDSA_KEY_PAIR_GEN: u32 = 4160;
pub const CKM_EC_KEY_PAIR_GEN: u32 = 4160;
pub const CKM_ECDSA: u32 = 4161;
pub const CKM_ECDSA_SHA1: u32 = 4162;
pub const CKM_ECDSA_SHA224: u32 = 4163;
pub const CKM_ECDSA_SHA256: u32 = 4164;
pub const CKM_ECDSA_SHA384: u32 = 4165;
pub const CKM_ECDSA_SHA512: u32 = 4166;
pub const CKM_ECDH1_DERIVE: u32 = 4176;
pub const CKM_ECDH1_COFACTOR_DERIVE: u32 = 4177;
pub const CKM_ECMQV_DERIVE: u32 = 4178;
pub const CKM_ECDH_AES_KEY_WRAP: u32 = 4179;
pub const CKM_RSA_AES_KEY_WRAP: u32 = 4180;
pub const CKM_JUNIPER_KEY_GEN: u32 = 4192;
pub const CKM_JUNIPER_ECB128: u32 = 4193;
pub const CKM_JUNIPER_CBC128: u32 = 4194;
pub const CKM_JUNIPER_COUNTER: u32 = 4195;
pub const CKM_JUNIPER_SHUFFLE: u32 = 4196;
pub const CKM_JUNIPER_WRAP: u32 = 4197;
pub const CKM_FASTHASH: u32 = 4208;
pub const CKM_AES_KEY_GEN: u32 = 4224;
pub const CKM_AES_ECB: u32 = 4225;
pub const CKM_AES_CBC: u32 = 4226;
pub const CKM_AES_MAC: u32 = 4227;
pub const CKM_AES_MAC_GENERAL: u32 = 4228;
pub const CKM_AES_CBC_PAD: u32 = 4229;
pub const CKM_AES_CTR: u32 = 4230;
pub const CKM_AES_GCM: u32 = 4231;
pub const CKM_AES_CCM: u32 = 4232;
pub const CKM_AES_CTS: u32 = 4233;
pub const CKM_AES_CMAC: u32 = 4234;
pub const CKM_AES_CMAC_GENERAL: u32 = 4235;
pub const CKM_AES_XCBC_MAC: u32 = 4236;
pub const CKM_AES_XCBC_MAC_96: u32 = 4237;
pub const CKM_AES_GMAC: u32 = 4238;
pub const CKM_BLOWFISH_KEY_GEN: u32 = 4240;
pub const CKM_BLOWFISH_CBC: u32 = 4241;
pub const CKM_TWOFISH_KEY_GEN: u32 = 4242;
pub const CKM_TWOFISH_CBC: u32 = 4243;
pub const CKM_BLOWFISH_CBC_PAD: u32 = 4244;
pub const CKM_TWOFISH_CBC_PAD: u32 = 4245;
pub const CKM_DES_ECB_ENCRYPT_DATA: u32 = 4352;
pub const CKM_DES_CBC_ENCRYPT_DATA: u32 = 4353;
pub const CKM_DES3_ECB_ENCRYPT_DATA: u32 = 4354;
pub const CKM_DES3_CBC_ENCRYPT_DATA: u32 = 4355;
pub const CKM_AES_ECB_ENCRYPT_DATA: u32 = 4356;
pub const CKM_AES_CBC_ENCRYPT_DATA: u32 = 4357;
pub const CKM_GOSTR3410_KEY_PAIR_GEN: u32 = 4608;
pub const CKM_GOSTR3410: u32 = 4609;
pub const CKM_GOSTR3410_WITH_GOSTR3411: u32 = 4610;
pub const CKM_GOSTR3410_KEY_WRAP: u32 = 4611;
pub const CKM_GOSTR3410_DERIVE: u32 = 4612;
pub const CKM_GOSTR3411: u32 = 4624;
pub const CKM_GOSTR3411_HMAC: u32 = 4625;
pub const CKM_GOST28147_KEY_GEN: u32 = 4640;
pub const CKM_GOST28147_ECB: u32 = 4641;
pub const CKM_GOST28147: u32 = 4642;
pub const CKM_GOST28147_MAC: u32 = 4643;
pub const CKM_GOST28147_KEY_WRAP: u32 = 4644;
pub const CKM_DSA_PARAMETER_GEN: u32 = 8192;
pub const CKM_DH_PKCS_PARAMETER_GEN: u32 = 8193;
pub const CKM_X9_42_DH_PARAMETER_GEN: u32 = 8194;
pub const CKM_DSA_PROBABLISTIC_PARAMETER_GEN: u32 = 8195;
pub const CKM_DSA_SHAWE_TAYLOR_PARAMETER_GEN: u32 = 8196;
pub const CKM_AES_OFB: u32 = 8452;
pub const CKM_AES_CFB64: u32 = 8453;
pub const CKM_AES_CFB8: u32 = 8454;
pub const CKM_AES_CFB128: u32 = 8455;
pub const CKM_AES_CFB1: u32 = 8456;
pub const CKM_SHA224: u32 = 597;
pub const CKM_SHA224_HMAC: u32 = 598;
pub const CKM_SHA224_HMAC_GENERAL: u32 = 599;
pub const CKM_SHA224_RSA_PKCS: u32 = 70;
pub const CKM_SHA224_RSA_PKCS_PSS: u32 = 71;
pub const CKM_SHA224_KEY_DERIVATION: u32 = 918;
pub const CKM_CAMELLIA_KEY_GEN: u32 = 1360;
pub const CKM_CAMELLIA_ECB: u32 = 1361;
pub const CKM_CAMELLIA_CBC: u32 = 1362;
pub const CKM_CAMELLIA_MAC: u32 = 1363;
pub const CKM_CAMELLIA_MAC_GENERAL: u32 = 1364;
pub const CKM_CAMELLIA_CBC_PAD: u32 = 1365;
pub const CKM_CAMELLIA_ECB_ENCRYPT_DATA: u32 = 1366;
pub const CKM_CAMELLIA_CBC_ENCRYPT_DATA: u32 = 1367;
pub const CKM_CAMELLIA_CTR: u32 = 1368;
pub const CKM_AES_KEY_WRAP: u32 = 8457;
pub const CKM_AES_KEY_WRAP_PAD: u32 = 8458;
pub const CKM_RSA_PKCS_TPM_1_1: u32 = 16385;
pub const CKM_RSA_PKCS_OAEP_TPM_1_1: u32 = 16386;
pub const CKM_EC_EDWARDS_KEY_PAIR_GEN: u32 = 4181;
pub const CKM_EDDSA: u32 = 4183;
pub const CK_OTP_FORMAT_DECIMAL: u32 = 0;
pub const CK_OTP_FORMAT_HEXADECIMAL: u32 = 1;
pub const CK_OTP_FORMAT_ALPHANUMERIC: u32 = 2;
pub const CK_OTP_FORMAT_BINARY: u32 = 3;
pub const CK_OTP_PARAM_IGNORED: u32 = 0;
pub const CK_OTP_PARAM_OPTIONAL: u32 = 1;
pub const CK_OTP_PARAM_MANDATORY: u32 = 2;
pub const CK_OTP_VALUE: u32 = 0;
pub const CK_OTP_PIN: u32 = 1;
pub const CK_OTP_CHALLENGE: u32 = 2;
pub const CK_OTP_TIME: u32 = 3;
pub const CK_OTP_COUNTER: u32 = 4;
pub const CK_OTP_FLAGS: u32 = 5;
pub const CK_OTP_OUTPUT_LENGTH: u32 = 6;
pub const CK_OTP_FORMAT: u32 = 7;
pub const CKF_NEXT_OTP: u32 = 1;
pub const CKF_EXCLUDE_TIME: u32 = 2;
pub const CKF_EXCLUDE_COUNTER: u32 = 4;
pub const CKF_EXCLUDE_CHALLENGE: u32 = 8;
pub const CKF_EXCLUDE_PIN: u32 = 16;
pub const CKF_USER_FRIENDLY_OTP: u32 = 32;
pub const CKN_OTP_CHANGED: u32 = 1;
pub const CK_SECURITY_DOMAIN_UNSPECIFIED: u32 = 0;
pub const CK_SECURITY_DOMAIN_MANUFACTURER: u32 = 1;
pub const CK_SECURITY_DOMAIN_OPERATOR: u32 = 2;
pub const CK_SECURITY_DOMAIN_THIRD_PARTY: u32 = 3;
pub const CK_CERTIFICATE_CATEGORY_UNSPECIFIED: u32 = 0;
pub const CK_CERTIFICATE_CATEGORY_TOKEN_USER: u32 = 1;
pub const CK_CERTIFICATE_CATEGORY_AUTHORITY: u32 = 2;
pub const CK_CERTIFICATE_CATEGORY_OTHER_ENTITY: u32 = 3;
pub const CKG_MGF1_SHA1: u32 = 1;
pub const CKG_MGF1_SHA224: u32 = 5;
pub const CKG_MGF1_SHA256: u32 = 2;
pub const CKG_MGF1_SHA384: u32 = 3;
pub const CKG_MGF1_SHA512: u32 = 4;
pub const CKD_NULL: u32 = 1;
pub const CKD_SHA1_KDF: u32 = 2;
pub const CKD_SHA1_KDF_ASN1: u32 = 3;
pub const CKD_SHA1_KDF_CONCATENATE: u32 = 4;
pub const CKD_SHA224_KDF: u32 = 5;
pub const CKD_SHA256_KDF: u32 = 6;
pub const CKD_SHA384_KDF: u32 = 7;
pub const CKD_SHA512_KDF: u32 = 8;
pub const CKD_CPDIVERSIFY_KDF: u32 = 9;
pub const CKF_HW: u32 = 1;
pub const CKF_ENCRYPT: u32 = 256;
pub const CKF_DECRYPT: u32 = 512;
pub const CKF_DIGEST: u32 = 1024;
pub const CKF_SIGN: u32 = 2048;
pub const CKF_SIGN_RECOVER: u32 = 4096;
pub const CKF_VERIFY: u32 = 8192;
pub const CKF_VERIFY_RECOVER: u32 = 16384;
pub const CKF_GENERATE: u32 = 32768;
pub const CKF_GENERATE_KEY_PAIR: u32 = 65536;
pub const CKF_WRAP: u32 = 131072;
pub const CKF_UNWRAP: u32 = 262144;
pub const CKF_DERIVE: u32 = 524288;
pub const CKF_EC_F_P: u32 = 1048576;
pub const CKF_EC_NAMEDCURVE: u32 = 8388608;
pub const CKF_EC_UNCOMPRESS: u32 = 16777216;
pub const CKF_EC_COMPRESS: u32 = 33554432;
pub const CKF_DONT_BLOCK: u32 = 1;
pub const CKF_LIBRARY_CANT_CREATE_OS_THREADS: u32 = 1;
pub const CKF_OS_LOCKING_OK: u32 = 2;
pub const CKR_OK: u32 = 0;
pub const CKR_CANCEL: u32 = 1;
pub const CKR_HOST_MEMORY: u32 = 2;
pub const CKR_SLOT_ID_INVALID: u32 = 3;
pub const CKR_GENERAL_ERROR: u32 = 5;
pub const CKR_FUNCTION_FAILED: u32 = 6;
pub const CKR_ARGUMENTS_BAD: u32 = 7;
pub const CKR_NO_EVENT: u32 = 8;
pub const CKR_NEED_TO_CREATE_THREADS: u32 = 9;
pub const CKR_CANT_LOCK: u32 = 10;
pub const CKR_ATTRIBUTE_READ_ONLY: u32 = 16;
pub const CKR_ATTRIBUTE_SENSITIVE: u32 = 17;
pub const CKR_ATTRIBUTE_TYPE_INVALID: u32 = 18;
pub const CKR_ATTRIBUTE_VALUE_INVALID: u32 = 19;
pub const CKR_ACTION_PROHIBITED: u32 = 27;
pub const CKR_DATA_INVALID: u32 = 32;
pub const CKR_DATA_LEN_RANGE: u32 = 33;
pub const CKR_DEVICE_ERROR: u32 = 48;
pub const CKR_DEVICE_MEMORY: u32 = 49;
pub const CKR_DEVICE_REMOVED: u32 = 50;
pub const CKR_ENCRYPTED_DATA_INVALID: u32 = 64;
pub const CKR_ENCRYPTED_DATA_LEN_RANGE: u32 = 65;
pub const CKR_FUNCTION_CANCELED: u32 = 80;
pub const CKR_FUNCTION_NOT_PARALLEL: u32 = 81;
pub const CKR_FUNCTION_NOT_SUPPORTED: u32 = 84;
pub const CKR_KEY_HANDLE_INVALID: u32 = 96;
pub const CKR_KEY_SIZE_RANGE: u32 = 98;
pub const CKR_KEY_TYPE_INCONSISTENT: u32 = 99;
pub const CKR_KEY_NOT_NEEDED: u32 = 100;
pub const CKR_KEY_CHANGED: u32 = 101;
pub const CKR_KEY_NEEDED: u32 = 102;
pub const CKR_KEY_INDIGESTIBLE: u32 = 103;
pub const CKR_KEY_FUNCTION_NOT_PERMITTED: u32 = 104;
pub const CKR_KEY_NOT_WRAPPABLE: u32 = 105;
pub const CKR_KEY_UNEXTRACTABLE: u32 = 106;
pub const CKR_MECHANISM_INVALID: u32 = 112;
pub const CKR_MECHANISM_PARAM_INVALID: u32 = 113;
pub const CKR_OBJECT_HANDLE_INVALID: u32 = 130;
pub const CKR_OPERATION_ACTIVE: u32 = 144;
pub const CKR_OPERATION_NOT_INITIALIZED: u32 = 145;
pub const CKR_PIN_INCORRECT: u32 = 160;
pub const CKR_PIN_INVALID: u32 = 161;
pub const CKR_PIN_LEN_RANGE: u32 = 162;
pub const CKR_PIN_EXPIRED: u32 = 163;
pub const CKR_PIN_LOCKED: u32 = 164;
pub const CKR_SESSION_CLOSED: u32 = 176;
pub const CKR_SESSION_COUNT: u32 = 177;
pub const CKR_SESSION_HANDLE_INVALID: u32 = 179;
pub const CKR_SESSION_PARALLEL_NOT_SUPPORTED: u32 = 180;
pub const CKR_SESSION_READ_ONLY: u32 = 181;
pub const CKR_SESSION_EXISTS: u32 = 182;
pub const CKR_SESSION_READ_ONLY_EXISTS: u32 = 183;
pub const CKR_SESSION_READ_WRITE_SO_EXISTS: u32 = 184;
pub const CKR_SIGNATURE_INVALID: u32 = 192;
pub const CKR_SIGNATURE_LEN_RANGE: u32 = 193;
pub const CKR_TEMPLATE_INCOMPLETE: u32 = 208;
pub const CKR_TEMPLATE_INCONSISTENT: u32 = 209;
pub const CKR_TOKEN_NOT_PRESENT: u32 = 224;
pub const CKR_TOKEN_NOT_RECOGNIZED: u32 = 225;
pub const CKR_TOKEN_WRITE_PROTECTED: u32 = 226;
pub const CKR_UNWRAPPING_KEY_HANDLE_INVALID: u32 = 240;
pub const CKR_UNWRAPPING_KEY_SIZE_RANGE: u32 = 241;
pub const CKR_UNWRAPPING_KEY_TYPE_INCONSISTENT: u32 = 242;
pub const CKR_USER_ALREADY_LOGGED_IN: u32 = 256;
pub const CKR_USER_NOT_LOGGED_IN: u32 = 257;
pub const CKR_USER_PIN_NOT_INITIALIZED: u32 = 258;
pub const CKR_USER_TYPE_INVALID: u32 = 259;
pub const CKR_USER_ANOTHER_ALREADY_LOGGED_IN: u32 = 260;
pub const CKR_USER_TOO_MANY_TYPES: u32 = 261;
pub const CKR_WRAPPED_KEY_INVALID: u32 = 272;
pub const CKR_WRAPPED_KEY_LEN_RANGE: u32 = 274;
pub const CKR_WRAPPING_KEY_HANDLE_INVALID: u32 = 275;
pub const CKR_WRAPPING_KEY_SIZE_RANGE: u32 = 276;
pub const CKR_WRAPPING_KEY_TYPE_INCONSISTENT: u32 = 277;
pub const CKR_RANDOM_SEED_NOT_SUPPORTED: u32 = 288;
pub const CKR_RANDOM_NO_RNG: u32 = 289;
pub const CKR_DOMAIN_PARAMS_INVALID: u32 = 304;
pub const CKR_CURVE_NOT_SUPPORTED: u32 = 320;
pub const CKR_BUFFER_TOO_SMALL: u32 = 336;
pub const CKR_SAVED_STATE_INVALID: u32 = 352;
pub const CKR_INFORMATION_SENSITIVE: u32 = 368;
pub const CKR_STATE_UNSAVEABLE: u32 = 384;
pub const CKR_CRYPTOKI_NOT_INITIALIZED: u32 = 400;
pub const CKR_CRYPTOKI_ALREADY_INITIALIZED: u32 = 401;
pub const CKR_MUTEX_BAD: u32 = 416;
pub const CKR_MUTEX_NOT_LOCKED: u32 = 417;
pub const CKR_NEW_PIN_MODE: u32 = 432;
pub const CKR_NEXT_OTP: u32 = 433;
pub const CKR_EXCEEDED_MAX_ITERATIONS: u32 = 448;
pub const CKR_FIPS_SELF_TEST_FAILED: u32 = 449;
pub const CKR_LIBRARY_LOAD_FAILED: u32 = 450;
pub const CKR_PIN_TOO_WEAK: u32 = 451;
pub const CKR_PUBLIC_KEY_INVALID: u32 = 452;
pub const CKR_FUNCTION_REJECTED: u32 = 512;
pub const CKZ_DATA_SPECIFIED: u32 = 1;
pub const CK_FALSE: u32 = 0;
pub const CK_TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const TRUE: u32 = 1;
pub type CK_FLAGS = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_VERSION {
    pub major: ::std::os::raw::c_uchar,
    pub minor: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_INFO {
    pub cryptokiVersion: _CK_VERSION,
    pub manufacturerID: [::std::os::raw::c_uchar; 32usize],
    pub flags: CK_FLAGS,
    pub libraryDescription: [::std::os::raw::c_uchar; 32usize],
    pub libraryVersion: _CK_VERSION,
}
pub type CK_NOTIFICATION = ::std::os::raw::c_ulong;
pub type CK_SLOT_ID = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_SLOT_INFO {
    pub slotDescription: [::std::os::raw::c_uchar; 64usize],
    pub manufacturerID: [::std::os::raw::c_uchar; 32usize],
    pub flags: CK_FLAGS,
    pub hardwareVersion: _CK_VERSION,
    pub firmwareVersion: _CK_VERSION,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_TOKEN_INFO {
    pub label: [::std::os::raw::c_uchar; 32usize],
    pub manufacturerID: [::std::os::raw::c_uchar; 32usize],
    pub model: [::std::os::raw::c_uchar; 16usize],
    pub serialNumber: [::std::os::raw::c_uchar; 16usize],
    pub flags: CK_FLAGS,
    pub ulMaxSessionCount: ::std::os::raw::c_ulong,
    pub ulSessionCount: ::std::os::raw::c_ulong,
    pub ulMaxRwSessionCount: ::std::os::raw::c_ulong,
    pub ulRwSessionCount: ::std::os::raw::c_ulong,
    pub ulMaxPinLen: ::std::os::raw::c_ulong,
    pub ulMinPinLen: ::std::os::raw::c_ulong,
    pub ulTotalPublicMemory: ::std::os::raw::c_ulong,
    pub ulFreePublicMemory: ::std::os::raw::c_ulong,
    pub ulTotalPrivateMemory: ::std::os::raw::c_ulong,
    pub ulFreePrivateMemory: ::std::os::raw::c_ulong,
    pub hardwareVersion: _CK_VERSION,
    pub firmwareVersion: _CK_VERSION,
    pub utcTime: [::std::os::raw::c_uchar; 16usize],
}
pub type CK_SESSION_HANDLE = ::std::os::raw::c_ulong;
pub type CK_USER_TYPE = ::std::os::raw::c_ulong;
pub type CK_STATE = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_SESSION_INFO {
    pub slotID: CK_SLOT_ID,
    pub state: CK_STATE,
    pub flags: CK_FLAGS,
    pub ulDeviceError: ::std::os::raw::c_ulong,
}
pub type CK_OBJECT_HANDLE = ::std::os::raw::c_ulong;
pub type CK_OBJECT_CLASS = ::std::os::raw::c_ulong;
pub type CK_HW_FEATURE_TYPE = ::std::os::raw::c_ulong;
pub type CK_KEY_TYPE = ::std::os::raw::c_ulong;
pub type CK_CERTIFICATE_TYPE = ::std::os::raw::c_ulong;
pub type CK_ATTRIBUTE_TYPE = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_ATTRIBUTE {
    pub type_: CK_ATTRIBUTE_TYPE,
    pub pValue: *mut ::std::os::raw::c_void,
    pub ulValueLen: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_DATE {
    pub year: [::std::os::raw::c_uchar; 4usize],
    pub month: [::std::os::raw::c_uchar; 2usize],
    pub day: [::std::os::raw::c_uchar; 2usize],
}
pub type CK_MECHANISM_TYPE = ::std::os::raw::c_ulong;
pub type CK_JAVA_MIDP_SECURITY_DOMAIN = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_MECHANISM {
    pub mechanism: CK_MECHANISM_TYPE,
    pub pParameter: *mut ::std::os::raw::c_void,
    pub ulParameterLen: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_MECHANISM_INFO {
    pub ulMinKeySize: ::std::os::raw::c_ulong,
    pub ulMaxKeySize: ::std::os::raw::c_ulong,
    pub flags: CK_FLAGS,
}
pub type CK_PARAM_TYPE = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_OTP_PARAM {
    pub type_: CK_PARAM_TYPE,
    pub pValue: *mut ::std::os::raw::c_void,
    pub ulValueLen: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_OTP_PARAMS {
    pub pParams: *mut CK_OTP_PARAM,
    pub ulCount: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_OTP_SIGNATURE_INFO {
    pub pParams: *mut CK_OTP_PARAM,
    pub ulCount: ::std::os::raw::c_ulong,
}
pub type CK_RSA_PKCS_MGF_TYPE = ::std::os::raw::c_ulong;
pub type CK_RSA_PKCS_MGF_TYPE_PTR = *mut CK_RSA_PKCS_MGF_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_rsa_pkcs_pss_params {
    pub hashAlg: CK_MECHANISM_TYPE,
    pub mgf: CK_RSA_PKCS_MGF_TYPE,
    pub sLen: ::std::os::raw::c_ulong,
}
pub type CK_RSA_PKCS_OAEP_SOURCE_TYPE = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_rsa_pkcs_oaep_params {
    pub hashAlg: CK_MECHANISM_TYPE,
    pub mgf: CK_RSA_PKCS_MGF_TYPE,
    pub source: CK_RSA_PKCS_OAEP_SOURCE_TYPE,
    pub pSourceData: *mut ::std::os::raw::c_void,
    pub ulSourceDataLen: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_aes_ctr_params {
    pub ulCounterBits: ::std::os::raw::c_ulong,
    pub cb: [::std::os::raw::c_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_gcm_params {
    pub pIv: *mut ::std::os::raw::c_uchar,
    pub ulIvLen: ::std::os::raw::c_ulong,
    pub ulIvBits: ::std::os::raw::c_ulong,
    pub pAAD: *mut ::std::os::raw::c_uchar,
    pub ulAADLen: ::std::os::raw::c_ulong,
    pub ulTagBits: ::std::os::raw::c_ulong,
}
pub type ck_ec_kdf_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_ecdh1_derive_params {
    pub kdf: ck_ec_kdf_t,
    pub ulSharedDataLen: ::std::os::raw::c_ulong,
    pub pSharedData: *mut ::std::os::raw::c_uchar,
    pub ulPublicDataLen: ::std::os::raw::c_ulong,
    pub pPublicData: *mut ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_key_derivation_string_data {
    pub pData: *mut ::std::os::raw::c_uchar,
    pub ulLen: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_des_cbc_encrypt_data_params {
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub pData: *mut ::std::os::raw::c_uchar,
    pub length: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_aes_cbc_encrypt_data_params {
    pub iv: [::std::os::raw::c_uchar; 16usize],
    pub pData: *mut ::std::os::raw::c_uchar,
    pub length: ::std::os::raw::c_ulong,
}
pub type CK_RV = ::std::os::raw::c_ulong;
pub type CK_NOTIFY = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        event: CK_NOTIFICATION,
        application: *mut ::std::os::raw::c_void,
    ) -> CK_RV,
>;
pub type CK_C_Initialize = ::std::option::Option<
    unsafe extern "C" fn(init_args: *mut ::std::os::raw::c_void) -> CK_RV,
>;
pub type C_Initialize_type = unsafe extern "C" fn(
    init_args: *mut ::std::os::raw::c_void,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Initialize(init_args: *mut ::std::os::raw::c_void) -> CK_RV {
    log::debug!("call C_Initialize");
    unsafe {
        let func: C_Initialize_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Initialize")
            .expect("fatal error: no symbol C_Initialize found in lib");
        func(init_args)
    }
}
pub type CK_C_Finalize = ::std::option::Option<
    unsafe extern "C" fn(pReserved: *mut ::std::os::raw::c_void) -> CK_RV,
>;
pub type C_Finalize_type = unsafe extern "C" fn(
    pReserved: *mut ::std::os::raw::c_void,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Finalize(pReserved: *mut ::std::os::raw::c_void) -> CK_RV {
    log::debug!("call C_Finalize");
    unsafe {
        let func: C_Finalize_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Finalize")
            .expect("fatal error: no symbol C_Finalize found in lib");
        func(pReserved)
    }
}
pub type CK_C_GetInfo = ::std::option::Option<
    unsafe extern "C" fn(info: *mut _CK_INFO) -> CK_RV,
>;
pub type C_GetInfo_type = unsafe extern "C" fn(info: *mut _CK_INFO) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetInfo(info: *mut _CK_INFO) -> CK_RV {
    log::debug!("call C_GetInfo");
    unsafe {
        let func: C_GetInfo_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetInfo")
            .expect("fatal error: no symbol C_GetInfo found in lib");
        func(info)
    }
}
pub type CK_C_GetFunctionList = ::std::option::Option<
    unsafe extern "C" fn(function_list: *mut *mut _CK_FUNCTION_LIST) -> CK_RV,
>;
pub type C_GetFunctionList_type = unsafe extern "C" fn(
    function_list: *mut *mut _CK_FUNCTION_LIST,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetFunctionList(
    function_list: *mut *mut _CK_FUNCTION_LIST,
) -> CK_RV {
    log::debug!("call C_GetFunctionList");
    unsafe {
        static mut STATIC_FUNCTION_LIST: _CK_FUNCTION_LIST = _CK_FUNCTION_LIST {
            version: _CK_VERSION { major: 0, minor: 0 },
            C_Initialize: Some(C_Initialize),
            C_Finalize: Some(C_Finalize),
            C_GetInfo: Some(C_GetInfo),
            C_GetFunctionList: Some(C_GetFunctionList),
            C_GetSlotList: Some(C_GetSlotList),
            C_GetSlotInfo: Some(C_GetSlotInfo),
            C_GetTokenInfo: Some(C_GetTokenInfo),
            C_GetMechanismList: Some(C_GetMechanismList),
            C_GetMechanismInfo: Some(C_GetMechanismInfo),
            C_InitToken: Some(C_InitToken),
            C_InitPIN: Some(C_InitPIN),
            C_SetPIN: Some(C_SetPIN),
            C_OpenSession: Some(C_OpenSession),
            C_CloseSession: Some(C_CloseSession),
            C_CloseAllSessions: Some(C_CloseAllSessions),
            C_GetSessionInfo: Some(C_GetSessionInfo),
            C_GetOperationState: Some(C_GetOperationState),
            C_SetOperationState: Some(C_SetOperationState),
            C_Login: Some(C_Login),
            C_Logout: Some(C_Logout),
            C_CreateObject: Some(C_CreateObject),
            C_CopyObject: Some(C_CopyObject),
            C_DestroyObject: Some(C_DestroyObject),
            C_GetObjectSize: Some(C_GetObjectSize),
            C_GetAttributeValue: Some(C_GetAttributeValue),
            C_SetAttributeValue: Some(C_SetAttributeValue),
            C_FindObjectsInit: Some(C_FindObjectsInit),
            C_FindObjects: Some(C_FindObjects),
            C_FindObjectsFinal: Some(C_FindObjectsFinal),
            C_EncryptInit: Some(C_EncryptInit),
            C_Encrypt: Some(C_Encrypt),
            C_EncryptUpdate: Some(C_EncryptUpdate),
            C_EncryptFinal: Some(C_EncryptFinal),
            C_DecryptInit: Some(C_DecryptInit),
            C_Decrypt: Some(C_Decrypt),
            C_DecryptUpdate: Some(C_DecryptUpdate),
            C_DecryptFinal: Some(C_DecryptFinal),
            C_DigestInit: Some(C_DigestInit),
            C_Digest: Some(C_Digest),
            C_DigestUpdate: Some(C_DigestUpdate),
            C_DigestKey: Some(C_DigestKey),
            C_DigestFinal: Some(C_DigestFinal),
            C_SignInit: Some(C_SignInit),
            C_Sign: Some(C_Sign),
            C_SignUpdate: Some(C_SignUpdate),
            C_SignFinal: Some(C_SignFinal),
            C_SignRecoverInit: Some(C_SignRecoverInit),
            C_SignRecover: Some(C_SignRecover),
            C_VerifyInit: Some(C_VerifyInit),
            C_Verify: Some(C_Verify),
            C_VerifyUpdate: Some(C_VerifyUpdate),
            C_VerifyFinal: Some(C_VerifyFinal),
            C_VerifyRecoverInit: Some(C_VerifyRecoverInit),
            C_VerifyRecover: Some(C_VerifyRecover),
            C_DigestEncryptUpdate: Some(C_DigestEncryptUpdate),
            C_DecryptDigestUpdate: Some(C_DecryptDigestUpdate),
            C_SignEncryptUpdate: Some(C_SignEncryptUpdate),
            C_DecryptVerifyUpdate: Some(C_DecryptVerifyUpdate),
            C_GenerateKey: Some(C_GenerateKey),
            C_GenerateKeyPair: Some(C_GenerateKeyPair),
            C_WrapKey: Some(C_WrapKey),
            C_UnwrapKey: Some(C_UnwrapKey),
            C_DeriveKey: Some(C_DeriveKey),
            C_SeedRandom: Some(C_SeedRandom),
            C_GenerateRandom: Some(C_GenerateRandom),
            C_GetFunctionStatus: Some(C_GetFunctionStatus),
            C_CancelFunction: Some(C_CancelFunction),
            C_WaitForSlotEvent: Some(C_WaitForSlotEvent),
        };
        *function_list = &mut STATIC_FUNCTION_LIST;
        CKR_OK as CK_RV
    }
}
pub type CK_C_GetSlotList = ::std::option::Option<
    unsafe extern "C" fn(
        token_present: ::std::os::raw::c_uchar,
        slot_list: *mut CK_SLOT_ID,
        ulCount: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GetSlotList_type = unsafe extern "C" fn(
    token_present: ::std::os::raw::c_uchar,
    slot_list: *mut CK_SLOT_ID,
    ulCount: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetSlotList(
    token_present: ::std::os::raw::c_uchar,
    slot_list: *mut CK_SLOT_ID,
    ulCount: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GetSlotList");
    unsafe {
        let func: C_GetSlotList_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetSlotList")
            .expect("fatal error: no symbol C_GetSlotList found in lib");
        func(token_present, slot_list, ulCount)
    }
}
pub type CK_C_GetSlotInfo = ::std::option::Option<
    unsafe extern "C" fn(slotID: CK_SLOT_ID, info: *mut _CK_SLOT_INFO) -> CK_RV,
>;
pub type C_GetSlotInfo_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    info: *mut _CK_SLOT_INFO,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetSlotInfo(slotID: CK_SLOT_ID, info: *mut _CK_SLOT_INFO) -> CK_RV {
    log::debug!("call C_GetSlotInfo");
    unsafe {
        let func: C_GetSlotInfo_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetSlotInfo")
            .expect("fatal error: no symbol C_GetSlotInfo found in lib");
        func(slotID, info)
    }
}
pub type CK_C_GetTokenInfo = ::std::option::Option<
    unsafe extern "C" fn(slotID: CK_SLOT_ID, info: *mut _CK_TOKEN_INFO) -> CK_RV,
>;
pub type C_GetTokenInfo_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    info: *mut _CK_TOKEN_INFO,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetTokenInfo(
    slotID: CK_SLOT_ID,
    info: *mut _CK_TOKEN_INFO,
) -> CK_RV {
    log::debug!("call C_GetTokenInfo");
    unsafe {
        let func: C_GetTokenInfo_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetTokenInfo")
            .expect("fatal error: no symbol C_GetTokenInfo found in lib");
        func(slotID, info)
    }
}
pub type CK_C_WaitForSlotEvent = ::std::option::Option<
    unsafe extern "C" fn(
        flags: CK_FLAGS,
        slot: *mut CK_SLOT_ID,
        pReserved: *mut ::std::os::raw::c_void,
    ) -> CK_RV,
>;
pub type C_WaitForSlotEvent_type = unsafe extern "C" fn(
    flags: CK_FLAGS,
    slot: *mut CK_SLOT_ID,
    pReserved: *mut ::std::os::raw::c_void,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_WaitForSlotEvent(
    flags: CK_FLAGS,
    slot: *mut CK_SLOT_ID,
    pReserved: *mut ::std::os::raw::c_void,
) -> CK_RV {
    log::debug!("call C_WaitForSlotEvent");
    unsafe {
        let func: C_WaitForSlotEvent_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_WaitForSlotEvent")
            .expect("fatal error: no symbol C_WaitForSlotEvent found in lib");
        func(flags, slot, pReserved)
    }
}
pub type CK_C_GetMechanismList = ::std::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        mechanism_list: *mut CK_MECHANISM_TYPE,
        ulCount: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GetMechanismList_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    mechanism_list: *mut CK_MECHANISM_TYPE,
    ulCount: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetMechanismList(
    slotID: CK_SLOT_ID,
    mechanism_list: *mut CK_MECHANISM_TYPE,
    ulCount: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GetMechanismList");
    unsafe {
        let func: C_GetMechanismList_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetMechanismList")
            .expect("fatal error: no symbol C_GetMechanismList found in lib");
        func(slotID, mechanism_list, ulCount)
    }
}
pub type CK_C_GetMechanismInfo = ::std::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        type_: CK_MECHANISM_TYPE,
        info: *mut _CK_MECHANISM_INFO,
    ) -> CK_RV,
>;
pub type C_GetMechanismInfo_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    type_: CK_MECHANISM_TYPE,
    info: *mut _CK_MECHANISM_INFO,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetMechanismInfo(
    slotID: CK_SLOT_ID,
    type_: CK_MECHANISM_TYPE,
    info: *mut _CK_MECHANISM_INFO,
) -> CK_RV {
    log::debug!("call C_GetMechanismInfo");
    unsafe {
        let func: C_GetMechanismInfo_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetMechanismInfo")
            .expect("fatal error: no symbol C_GetMechanismInfo found in lib");
        func(slotID, type_, info)
    }
}
pub type CK_C_InitToken = ::std::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        pin: *mut ::std::os::raw::c_uchar,
        pin_len: ::std::os::raw::c_ulong,
        label: *mut ::std::os::raw::c_uchar,
    ) -> CK_RV,
>;
pub type C_InitToken_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
    label: *mut ::std::os::raw::c_uchar,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_InitToken(
    slotID: CK_SLOT_ID,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
    label: *mut ::std::os::raw::c_uchar,
) -> CK_RV {
    log::debug!("call C_InitToken");
    unsafe {
        let func: C_InitToken_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_InitToken")
            .expect("fatal error: no symbol C_InitToken found in lib");
        func(slotID, pin, pin_len, label)
    }
}
pub type CK_C_InitPIN = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        pin: *mut ::std::os::raw::c_uchar,
        pin_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_InitPIN_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_InitPIN(
    session: CK_SESSION_HANDLE,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_InitPIN");
    unsafe {
        let func: C_InitPIN_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_InitPIN")
            .expect("fatal error: no symbol C_InitPIN found in lib");
        func(session, pin, pin_len)
    }
}
pub type CK_C_SetPIN = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        old_pin: *mut ::std::os::raw::c_uchar,
        old_len: ::std::os::raw::c_ulong,
        new_pin: *mut ::std::os::raw::c_uchar,
        new_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SetPIN_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    old_pin: *mut ::std::os::raw::c_uchar,
    old_len: ::std::os::raw::c_ulong,
    new_pin: *mut ::std::os::raw::c_uchar,
    new_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SetPIN(
    session: CK_SESSION_HANDLE,
    old_pin: *mut ::std::os::raw::c_uchar,
    old_len: ::std::os::raw::c_ulong,
    new_pin: *mut ::std::os::raw::c_uchar,
    new_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SetPIN");
    unsafe {
        let func: C_SetPIN_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SetPIN")
            .expect("fatal error: no symbol C_SetPIN found in lib");
        func(session, old_pin, old_len, new_pin, new_len)
    }
}
pub type CK_C_OpenSession = ::std::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        flags: CK_FLAGS,
        application: *mut ::std::os::raw::c_void,
        notify: CK_NOTIFY,
        session: *mut CK_SESSION_HANDLE,
    ) -> CK_RV,
>;
pub type C_OpenSession_type = unsafe extern "C" fn(
    slotID: CK_SLOT_ID,
    flags: CK_FLAGS,
    application: *mut ::std::os::raw::c_void,
    notify: CK_NOTIFY,
    session: *mut CK_SESSION_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_OpenSession(
    slotID: CK_SLOT_ID,
    flags: CK_FLAGS,
    application: *mut ::std::os::raw::c_void,
    notify: CK_NOTIFY,
    session: *mut CK_SESSION_HANDLE,
) -> CK_RV {
    log::debug!("call C_OpenSession");
    unsafe {
        let func: C_OpenSession_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_OpenSession")
            .expect("fatal error: no symbol C_OpenSession found in lib");
        func(slotID, flags, application, notify, session)
    }
}
pub type CK_C_CloseSession = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV,
>;
pub type C_CloseSession_type = unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_CloseSession(session: CK_SESSION_HANDLE) -> CK_RV {
    log::debug!("call C_CloseSession");
    unsafe {
        let func: C_CloseSession_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_CloseSession")
            .expect("fatal error: no symbol C_CloseSession found in lib");
        func(session)
    }
}
pub type CK_C_CloseAllSessions = ::std::option::Option<
    unsafe extern "C" fn(slotID: CK_SLOT_ID) -> CK_RV,
>;
pub type C_CloseAllSessions_type = unsafe extern "C" fn(slotID: CK_SLOT_ID) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_CloseAllSessions(slotID: CK_SLOT_ID) -> CK_RV {
    log::debug!("call C_CloseAllSessions");
    unsafe {
        let func: C_CloseAllSessions_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_CloseAllSessions")
            .expect("fatal error: no symbol C_CloseAllSessions found in lib");
        func(slotID)
    }
}
pub type CK_C_GetSessionInfo = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        info: *mut _CK_SESSION_INFO,
    ) -> CK_RV,
>;
pub type C_GetSessionInfo_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    info: *mut _CK_SESSION_INFO,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetSessionInfo(
    session: CK_SESSION_HANDLE,
    info: *mut _CK_SESSION_INFO,
) -> CK_RV {
    log::debug!("call C_GetSessionInfo");
    unsafe {
        let func: C_GetSessionInfo_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetSessionInfo")
            .expect("fatal error: no symbol C_GetSessionInfo found in lib");
        func(session, info)
    }
}
pub type CK_C_GetOperationState = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        operation_state: *mut ::std::os::raw::c_uchar,
        operation_state_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GetOperationState_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    operation_state: *mut ::std::os::raw::c_uchar,
    operation_state_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetOperationState(
    session: CK_SESSION_HANDLE,
    operation_state: *mut ::std::os::raw::c_uchar,
    operation_state_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GetOperationState");
    unsafe {
        let func: C_GetOperationState_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetOperationState")
            .expect("fatal error: no symbol C_GetOperationState found in lib");
        func(session, operation_state, operation_state_len)
    }
}
pub type CK_C_SetOperationState = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        operation_state: *mut ::std::os::raw::c_uchar,
        operation_state_len: ::std::os::raw::c_ulong,
        encryption_key: CK_OBJECT_HANDLE,
        authentiation_key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_SetOperationState_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    operation_state: *mut ::std::os::raw::c_uchar,
    operation_state_len: ::std::os::raw::c_ulong,
    encryption_key: CK_OBJECT_HANDLE,
    authentiation_key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SetOperationState(
    session: CK_SESSION_HANDLE,
    operation_state: *mut ::std::os::raw::c_uchar,
    operation_state_len: ::std::os::raw::c_ulong,
    encryption_key: CK_OBJECT_HANDLE,
    authentiation_key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_SetOperationState");
    unsafe {
        let func: C_SetOperationState_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SetOperationState")
            .expect("fatal error: no symbol C_SetOperationState found in lib");
        func(
            session,
            operation_state,
            operation_state_len,
            encryption_key,
            authentiation_key,
        )
    }
}
pub type CK_C_Login = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        user_type: CK_USER_TYPE,
        pin: *mut ::std::os::raw::c_uchar,
        pin_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Login_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    user_type: CK_USER_TYPE,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Login(
    session: CK_SESSION_HANDLE,
    user_type: CK_USER_TYPE,
    pin: *mut ::std::os::raw::c_uchar,
    pin_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Login");
    unsafe {
        let func: C_Login_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Login")
            .expect("fatal error: no symbol C_Login found in lib");
        func(session, user_type, pin, pin_len)
    }
}
pub type CK_C_Logout = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV,
>;
pub type C_Logout_type = unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Logout(session: CK_SESSION_HANDLE) -> CK_RV {
    log::debug!("call C_Logout");
    unsafe {
        let func: C_Logout_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Logout")
            .expect("fatal error: no symbol C_Logout found in lib");
        func(session)
    }
}
pub type CK_C_CreateObject = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
        object: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_CreateObject_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    object: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_CreateObject(
    session: CK_SESSION_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    object: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_CreateObject");
    unsafe {
        let func: C_CreateObject_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_CreateObject")
            .expect("fatal error: no symbol C_CreateObject found in lib");
        func(session, templ, ulCount, object)
    }
}
pub type CK_C_CopyObject = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        object: CK_OBJECT_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
        new_object: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_CopyObject_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    new_object: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_CopyObject(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    new_object: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_CopyObject");
    unsafe {
        let func: C_CopyObject_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_CopyObject")
            .expect("fatal error: no symbol C_CopyObject found in lib");
        func(session, object, templ, ulCount, new_object)
    }
}
pub type CK_C_DestroyObject = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE, object: CK_OBJECT_HANDLE) -> CK_RV,
>;
pub type C_DestroyObject_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DestroyObject(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_DestroyObject");
    unsafe {
        let func: C_DestroyObject_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DestroyObject")
            .expect("fatal error: no symbol C_DestroyObject found in lib");
        func(session, object)
    }
}
pub type CK_C_GetObjectSize = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        object: CK_OBJECT_HANDLE,
        size: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GetObjectSize_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    size: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetObjectSize(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    size: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GetObjectSize");
    unsafe {
        let func: C_GetObjectSize_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetObjectSize")
            .expect("fatal error: no symbol C_GetObjectSize found in lib");
        func(session, object, size)
    }
}
pub type CK_C_GetAttributeValue = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        object: CK_OBJECT_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GetAttributeValue_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetAttributeValue(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GetAttributeValue");
    unsafe {
        let func: C_GetAttributeValue_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetAttributeValue")
            .expect("fatal error: no symbol C_GetAttributeValue found in lib");
        func(session, object, templ, ulCount)
    }
}
pub type CK_C_SetAttributeValue = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        object: CK_OBJECT_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SetAttributeValue_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SetAttributeValue(
    session: CK_SESSION_HANDLE,
    object: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SetAttributeValue");
    unsafe {
        let func: C_SetAttributeValue_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SetAttributeValue")
            .expect("fatal error: no symbol C_SetAttributeValue found in lib");
        func(session, object, templ, ulCount)
    }
}
pub type CK_C_FindObjectsInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_FindObjectsInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_FindObjectsInit(
    session: CK_SESSION_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_FindObjectsInit");
    unsafe {
        let func: C_FindObjectsInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_FindObjectsInit")
            .expect("fatal error: no symbol C_FindObjectsInit found in lib");
        func(session, templ, ulCount)
    }
}
pub type CK_C_FindObjects = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        object: *mut CK_OBJECT_HANDLE,
        max_object_count: ::std::os::raw::c_ulong,
        object_count: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_FindObjects_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    object: *mut CK_OBJECT_HANDLE,
    max_object_count: ::std::os::raw::c_ulong,
    object_count: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_FindObjects(
    session: CK_SESSION_HANDLE,
    object: *mut CK_OBJECT_HANDLE,
    max_object_count: ::std::os::raw::c_ulong,
    object_count: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_FindObjects");
    unsafe {
        let func: C_FindObjects_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_FindObjects")
            .expect("fatal error: no symbol C_FindObjects found in lib");
        func(session, object, max_object_count, object_count)
    }
}
pub type CK_C_FindObjectsFinal = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV,
>;
pub type C_FindObjectsFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_FindObjectsFinal(session: CK_SESSION_HANDLE) -> CK_RV {
    log::debug!("call C_FindObjectsFinal");
    unsafe {
        let func: C_FindObjectsFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_FindObjectsFinal")
            .expect("fatal error: no symbol C_FindObjectsFinal found in lib");
        func(session)
    }
}
pub type CK_C_EncryptInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_EncryptInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_EncryptInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_EncryptInit");
    unsafe {
        let func: C_EncryptInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_EncryptInit")
            .expect("fatal error: no symbol C_EncryptInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_Encrypt = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        data: *mut ::std::os::raw::c_uchar,
        data_len: ::std::os::raw::c_ulong,
        encrypted_data: *mut ::std::os::raw::c_uchar,
        encrypted_data_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Encrypt_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    encrypted_data: *mut ::std::os::raw::c_uchar,
    encrypted_data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Encrypt(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    encrypted_data: *mut ::std::os::raw::c_uchar,
    encrypted_data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Encrypt");
    unsafe {
        let func: C_Encrypt_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Encrypt")
            .expect("fatal error: no symbol C_Encrypt found in lib");
        func(session, data, data_len, encrypted_data, encrypted_data_len)
    }
}
pub type CK_C_EncryptUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_EncryptUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_EncryptUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_EncryptUpdate");
    unsafe {
        let func: C_EncryptUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_EncryptUpdate")
            .expect("fatal error: no symbol C_EncryptUpdate found in lib");
        func(session, part, part_len, encrypted_part, encrypted_part_len)
    }
}
pub type CK_C_EncryptFinal = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        last_encrypted_part: *mut ::std::os::raw::c_uchar,
        last_encrypted_part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_EncryptFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    last_encrypted_part: *mut ::std::os::raw::c_uchar,
    last_encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_EncryptFinal(
    session: CK_SESSION_HANDLE,
    last_encrypted_part: *mut ::std::os::raw::c_uchar,
    last_encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_EncryptFinal");
    unsafe {
        let func: C_EncryptFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_EncryptFinal")
            .expect("fatal error: no symbol C_EncryptFinal found in lib");
        func(session, last_encrypted_part, last_encrypted_part_len)
    }
}
pub type CK_C_DecryptInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_DecryptInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DecryptInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_DecryptInit");
    unsafe {
        let func: C_DecryptInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DecryptInit")
            .expect("fatal error: no symbol C_DecryptInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_Decrypt = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        encrypted_data: *mut ::std::os::raw::c_uchar,
        encrypted_data_len: ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_uchar,
        data_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Decrypt_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    encrypted_data: *mut ::std::os::raw::c_uchar,
    encrypted_data_len: ::std::os::raw::c_ulong,
    data: *mut ::std::os::raw::c_uchar,
    data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Decrypt(
    session: CK_SESSION_HANDLE,
    encrypted_data: *mut ::std::os::raw::c_uchar,
    encrypted_data_len: ::std::os::raw::c_ulong,
    data: *mut ::std::os::raw::c_uchar,
    data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Decrypt");
    unsafe {
        let func: C_Decrypt_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Decrypt")
            .expect("fatal error: no symbol C_Decrypt found in lib");
        func(session, encrypted_data, encrypted_data_len, data, data_len)
    }
}
pub type CK_C_DecryptUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: ::std::os::raw::c_ulong,
        part: *mut ::std::os::raw::c_uchar,
        part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DecryptUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DecryptUpdate(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DecryptUpdate");
    unsafe {
        let func: C_DecryptUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DecryptUpdate")
            .expect("fatal error: no symbol C_DecryptUpdate found in lib");
        func(session, encrypted_part, encrypted_part_len, part, part_len)
    }
}
pub type CK_C_DecryptFinal = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        last_part: *mut ::std::os::raw::c_uchar,
        last_part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DecryptFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    last_part: *mut ::std::os::raw::c_uchar,
    last_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DecryptFinal(
    session: CK_SESSION_HANDLE,
    last_part: *mut ::std::os::raw::c_uchar,
    last_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DecryptFinal");
    unsafe {
        let func: C_DecryptFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DecryptFinal")
            .expect("fatal error: no symbol C_DecryptFinal found in lib");
        func(session, last_part, last_part_len)
    }
}
pub type CK_C_DigestInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
    ) -> CK_RV,
>;
pub type C_DigestInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DigestInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
) -> CK_RV {
    log::debug!("call C_DigestInit");
    unsafe {
        let func: C_DigestInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DigestInit")
            .expect("fatal error: no symbol C_DigestInit found in lib");
        func(session, mechanism)
    }
}
pub type CK_C_Digest = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        data: *mut ::std::os::raw::c_uchar,
        data_len: ::std::os::raw::c_ulong,
        digest: *mut ::std::os::raw::c_uchar,
        digest_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Digest_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    digest: *mut ::std::os::raw::c_uchar,
    digest_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Digest(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    digest: *mut ::std::os::raw::c_uchar,
    digest_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Digest");
    unsafe {
        let func: C_Digest_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Digest")
            .expect("fatal error: no symbol C_Digest found in lib");
        func(session, data, data_len, digest, digest_len)
    }
}
pub type CK_C_DigestUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DigestUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DigestUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DigestUpdate");
    unsafe {
        let func: C_DigestUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DigestUpdate")
            .expect("fatal error: no symbol C_DigestUpdate found in lib");
        func(session, part, part_len)
    }
}
pub type CK_C_DigestKey = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE, key: CK_OBJECT_HANDLE) -> CK_RV,
>;
pub type C_DigestKey_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DigestKey(
    session: CK_SESSION_HANDLE,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_DigestKey");
    unsafe {
        let func: C_DigestKey_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DigestKey")
            .expect("fatal error: no symbol C_DigestKey found in lib");
        func(session, key)
    }
}
pub type CK_C_DigestFinal = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        digest: *mut ::std::os::raw::c_uchar,
        digest_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DigestFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    digest: *mut ::std::os::raw::c_uchar,
    digest_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DigestFinal(
    session: CK_SESSION_HANDLE,
    digest: *mut ::std::os::raw::c_uchar,
    digest_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DigestFinal");
    unsafe {
        let func: C_DigestFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DigestFinal")
            .expect("fatal error: no symbol C_DigestFinal found in lib");
        func(session, digest, digest_len)
    }
}
pub type CK_C_SignInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_SignInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_SignInit");
    unsafe {
        let func: C_SignInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignInit")
            .expect("fatal error: no symbol C_SignInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_Sign = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        data: *mut ::std::os::raw::c_uchar,
        data_len: ::std::os::raw::c_ulong,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Sign_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Sign(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Sign");
    unsafe {
        let func: C_Sign_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Sign")
            .expect("fatal error: no symbol C_Sign found in lib");
        func(session, data, data_len, signature, signature_len)
    }
}
pub type CK_C_SignUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SignUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SignUpdate");
    unsafe {
        let func: C_SignUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignUpdate")
            .expect("fatal error: no symbol C_SignUpdate found in lib");
        func(session, part, part_len)
    }
}
pub type CK_C_SignFinal = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SignFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignFinal(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SignFinal");
    unsafe {
        let func: C_SignFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignFinal")
            .expect("fatal error: no symbol C_SignFinal found in lib");
        func(session, signature, signature_len)
    }
}
pub type CK_C_SignRecoverInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_SignRecoverInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignRecoverInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_SignRecoverInit");
    unsafe {
        let func: C_SignRecoverInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignRecoverInit")
            .expect("fatal error: no symbol C_SignRecoverInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_SignRecover = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        data: *mut ::std::os::raw::c_uchar,
        data_len: ::std::os::raw::c_ulong,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SignRecover_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignRecover(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SignRecover");
    unsafe {
        let func: C_SignRecover_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignRecover")
            .expect("fatal error: no symbol C_SignRecover found in lib");
        func(session, data, data_len, signature, signature_len)
    }
}
pub type CK_C_VerifyInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_VerifyInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_VerifyInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_VerifyInit");
    unsafe {
        let func: C_VerifyInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_VerifyInit")
            .expect("fatal error: no symbol C_VerifyInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_Verify = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        data: *mut ::std::os::raw::c_uchar,
        data_len: ::std::os::raw::c_ulong,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_Verify_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_Verify(
    session: CK_SESSION_HANDLE,
    data: *mut ::std::os::raw::c_uchar,
    data_len: ::std::os::raw::c_ulong,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_Verify");
    unsafe {
        let func: C_Verify_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_Verify")
            .expect("fatal error: no symbol C_Verify found in lib");
        func(session, data, data_len, signature, signature_len)
    }
}
pub type CK_C_VerifyUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_VerifyUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_VerifyUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_VerifyUpdate");
    unsafe {
        let func: C_VerifyUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_VerifyUpdate")
            .expect("fatal error: no symbol C_VerifyUpdate found in lib");
        func(session, part, part_len)
    }
}
pub type CK_C_VerifyFinal = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_VerifyFinal_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_VerifyFinal(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_VerifyFinal");
    unsafe {
        let func: C_VerifyFinal_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_VerifyFinal")
            .expect("fatal error: no symbol C_VerifyFinal found in lib");
        func(session, signature, signature_len)
    }
}
pub type CK_C_VerifyRecoverInit = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        key: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_VerifyRecoverInit_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_VerifyRecoverInit(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    key: CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_VerifyRecoverInit");
    unsafe {
        let func: C_VerifyRecoverInit_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_VerifyRecoverInit")
            .expect("fatal error: no symbol C_VerifyRecoverInit found in lib");
        func(session, mechanism, key)
    }
}
pub type CK_C_VerifyRecover = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        signature: *mut ::std::os::raw::c_uchar,
        signature_len: ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_uchar,
        data_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_VerifyRecover_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
    data: *mut ::std::os::raw::c_uchar,
    data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_VerifyRecover(
    session: CK_SESSION_HANDLE,
    signature: *mut ::std::os::raw::c_uchar,
    signature_len: ::std::os::raw::c_ulong,
    data: *mut ::std::os::raw::c_uchar,
    data_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_VerifyRecover");
    unsafe {
        let func: C_VerifyRecover_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_VerifyRecover")
            .expect("fatal error: no symbol C_VerifyRecover found in lib");
        func(session, signature, signature_len, data, data_len)
    }
}
pub type CK_C_DigestEncryptUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DigestEncryptUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DigestEncryptUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DigestEncryptUpdate");
    unsafe {
        let func: C_DigestEncryptUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DigestEncryptUpdate")
            .expect("fatal error: no symbol C_DigestEncryptUpdate found in lib");
        func(session, part, part_len, encrypted_part, encrypted_part_len)
    }
}
pub type CK_C_DecryptDigestUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: ::std::os::raw::c_ulong,
        part: *mut ::std::os::raw::c_uchar,
        part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DecryptDigestUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DecryptDigestUpdate(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DecryptDigestUpdate");
    unsafe {
        let func: C_DecryptDigestUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DecryptDigestUpdate")
            .expect("fatal error: no symbol C_DecryptDigestUpdate found in lib");
        func(session, encrypted_part, encrypted_part_len, part, part_len)
    }
}
pub type CK_C_SignEncryptUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        part: *mut ::std::os::raw::c_uchar,
        part_len: ::std::os::raw::c_ulong,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SignEncryptUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SignEncryptUpdate(
    session: CK_SESSION_HANDLE,
    part: *mut ::std::os::raw::c_uchar,
    part_len: ::std::os::raw::c_ulong,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SignEncryptUpdate");
    unsafe {
        let func: C_SignEncryptUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SignEncryptUpdate")
            .expect("fatal error: no symbol C_SignEncryptUpdate found in lib");
        func(session, part, part_len, encrypted_part, encrypted_part_len)
    }
}
pub type CK_C_DecryptVerifyUpdate = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        encrypted_part: *mut ::std::os::raw::c_uchar,
        encrypted_part_len: ::std::os::raw::c_ulong,
        part: *mut ::std::os::raw::c_uchar,
        part_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_DecryptVerifyUpdate_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DecryptVerifyUpdate(
    session: CK_SESSION_HANDLE,
    encrypted_part: *mut ::std::os::raw::c_uchar,
    encrypted_part_len: ::std::os::raw::c_ulong,
    part: *mut ::std::os::raw::c_uchar,
    part_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_DecryptVerifyUpdate");
    unsafe {
        let func: C_DecryptVerifyUpdate_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DecryptVerifyUpdate")
            .expect("fatal error: no symbol C_DecryptVerifyUpdate found in lib");
        func(session, encrypted_part, encrypted_part_len, part, part_len)
    }
}
pub type CK_C_GenerateKey = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        templ: *mut _CK_ATTRIBUTE,
        ulCount: ::std::os::raw::c_ulong,
        key: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_GenerateKey_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GenerateKey(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    templ: *mut _CK_ATTRIBUTE,
    ulCount: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_GenerateKey");
    unsafe {
        let func: C_GenerateKey_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GenerateKey")
            .expect("fatal error: no symbol C_GenerateKey found in lib");
        func(session, mechanism, templ, ulCount, key)
    }
}
pub type CK_C_GenerateKeyPair = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        public_key_template: *mut _CK_ATTRIBUTE,
        public_key_attribute_count: ::std::os::raw::c_ulong,
        private_key_template: *mut _CK_ATTRIBUTE,
        private_key_attribute_count: ::std::os::raw::c_ulong,
        public_key: *mut CK_OBJECT_HANDLE,
        private_key: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_GenerateKeyPair_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    public_key_template: *mut _CK_ATTRIBUTE,
    public_key_attribute_count: ::std::os::raw::c_ulong,
    private_key_template: *mut _CK_ATTRIBUTE,
    private_key_attribute_count: ::std::os::raw::c_ulong,
    public_key: *mut CK_OBJECT_HANDLE,
    private_key: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GenerateKeyPair(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    public_key_template: *mut _CK_ATTRIBUTE,
    public_key_attribute_count: ::std::os::raw::c_ulong,
    private_key_template: *mut _CK_ATTRIBUTE,
    private_key_attribute_count: ::std::os::raw::c_ulong,
    public_key: *mut CK_OBJECT_HANDLE,
    private_key: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_GenerateKeyPair");
    unsafe {
        let func: C_GenerateKeyPair_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GenerateKeyPair")
            .expect("fatal error: no symbol C_GenerateKeyPair found in lib");
        func(
            session,
            mechanism,
            public_key_template,
            public_key_attribute_count,
            private_key_template,
            private_key_attribute_count,
            public_key,
            private_key,
        )
    }
}
pub type CK_C_WrapKey = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        wrapping_key: CK_OBJECT_HANDLE,
        key: CK_OBJECT_HANDLE,
        wrapped_key: *mut ::std::os::raw::c_uchar,
        wrapped_key_len: *mut ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_WrapKey_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    wrapping_key: CK_OBJECT_HANDLE,
    key: CK_OBJECT_HANDLE,
    wrapped_key: *mut ::std::os::raw::c_uchar,
    wrapped_key_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_WrapKey(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    wrapping_key: CK_OBJECT_HANDLE,
    key: CK_OBJECT_HANDLE,
    wrapped_key: *mut ::std::os::raw::c_uchar,
    wrapped_key_len: *mut ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_WrapKey");
    unsafe {
        let func: C_WrapKey_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_WrapKey")
            .expect("fatal error: no symbol C_WrapKey found in lib");
        func(session, mechanism, wrapping_key, key, wrapped_key, wrapped_key_len)
    }
}
pub type CK_C_UnwrapKey = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        unwrapping_key: CK_OBJECT_HANDLE,
        wrapped_key: *mut ::std::os::raw::c_uchar,
        wrapped_key_len: ::std::os::raw::c_ulong,
        templ: *mut _CK_ATTRIBUTE,
        attribute_count: ::std::os::raw::c_ulong,
        key: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_UnwrapKey_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    unwrapping_key: CK_OBJECT_HANDLE,
    wrapped_key: *mut ::std::os::raw::c_uchar,
    wrapped_key_len: ::std::os::raw::c_ulong,
    templ: *mut _CK_ATTRIBUTE,
    attribute_count: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_UnwrapKey(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    unwrapping_key: CK_OBJECT_HANDLE,
    wrapped_key: *mut ::std::os::raw::c_uchar,
    wrapped_key_len: ::std::os::raw::c_ulong,
    templ: *mut _CK_ATTRIBUTE,
    attribute_count: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_UnwrapKey");
    unsafe {
        let func: C_UnwrapKey_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_UnwrapKey")
            .expect("fatal error: no symbol C_UnwrapKey found in lib");
        func(
            session,
            mechanism,
            unwrapping_key,
            wrapped_key,
            wrapped_key_len,
            templ,
            attribute_count,
            key,
        )
    }
}
pub type CK_C_DeriveKey = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        mechanism: *mut _CK_MECHANISM,
        base_key: CK_OBJECT_HANDLE,
        templ: *mut _CK_ATTRIBUTE,
        attribute_count: ::std::os::raw::c_ulong,
        key: *mut CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;
pub type C_DeriveKey_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    base_key: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    attribute_count: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_DeriveKey(
    session: CK_SESSION_HANDLE,
    mechanism: *mut _CK_MECHANISM,
    base_key: CK_OBJECT_HANDLE,
    templ: *mut _CK_ATTRIBUTE,
    attribute_count: ::std::os::raw::c_ulong,
    key: *mut CK_OBJECT_HANDLE,
) -> CK_RV {
    log::debug!("call C_DeriveKey");
    unsafe {
        let func: C_DeriveKey_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_DeriveKey")
            .expect("fatal error: no symbol C_DeriveKey found in lib");
        func(session, mechanism, base_key, templ, attribute_count, key)
    }
}
pub type CK_C_SeedRandom = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        seed: *mut ::std::os::raw::c_uchar,
        seed_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_SeedRandom_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    seed: *mut ::std::os::raw::c_uchar,
    seed_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_SeedRandom(
    session: CK_SESSION_HANDLE,
    seed: *mut ::std::os::raw::c_uchar,
    seed_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_SeedRandom");
    unsafe {
        let func: C_SeedRandom_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_SeedRandom")
            .expect("fatal error: no symbol C_SeedRandom found in lib");
        func(session, seed, seed_len)
    }
}
pub type CK_C_GenerateRandom = ::std::option::Option<
    unsafe extern "C" fn(
        session: CK_SESSION_HANDLE,
        random_data: *mut ::std::os::raw::c_uchar,
        random_len: ::std::os::raw::c_ulong,
    ) -> CK_RV,
>;
pub type C_GenerateRandom_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
    random_data: *mut ::std::os::raw::c_uchar,
    random_len: ::std::os::raw::c_ulong,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GenerateRandom(
    session: CK_SESSION_HANDLE,
    random_data: *mut ::std::os::raw::c_uchar,
    random_len: ::std::os::raw::c_ulong,
) -> CK_RV {
    log::debug!("call C_GenerateRandom");
    unsafe {
        let func: C_GenerateRandom_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GenerateRandom")
            .expect("fatal error: no symbol C_GenerateRandom found in lib");
        func(session, random_data, random_len)
    }
}
pub type CK_C_GetFunctionStatus = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV,
>;
pub type C_GetFunctionStatus_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_GetFunctionStatus(session: CK_SESSION_HANDLE) -> CK_RV {
    log::debug!("call C_GetFunctionStatus");
    unsafe {
        let func: C_GetFunctionStatus_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_GetFunctionStatus")
            .expect("fatal error: no symbol C_GetFunctionStatus found in lib");
        func(session)
    }
}
pub type CK_C_CancelFunction = ::std::option::Option<
    unsafe extern "C" fn(session: CK_SESSION_HANDLE) -> CK_RV,
>;
pub type C_CancelFunction_type = unsafe extern "C" fn(
    session: CK_SESSION_HANDLE,
) -> CK_RV;
#[no_mangle]
pub extern "C" fn C_CancelFunction(session: CK_SESSION_HANDLE) -> CK_RV {
    log::debug!("call C_CancelFunction");
    unsafe {
        let func: C_CancelFunction_type = crate::overrides::get_lw()
            .read()
            .unwrap()
            .get(b"C_CancelFunction")
            .expect("fatal error: no symbol C_CancelFunction found in lib");
        func(session)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_FUNCTION_LIST {
    pub version: _CK_VERSION,
    pub C_Initialize: CK_C_Initialize,
    pub C_Finalize: CK_C_Finalize,
    pub C_GetInfo: CK_C_GetInfo,
    pub C_GetFunctionList: CK_C_GetFunctionList,
    pub C_GetSlotList: CK_C_GetSlotList,
    pub C_GetSlotInfo: CK_C_GetSlotInfo,
    pub C_GetTokenInfo: CK_C_GetTokenInfo,
    pub C_GetMechanismList: CK_C_GetMechanismList,
    pub C_GetMechanismInfo: CK_C_GetMechanismInfo,
    pub C_InitToken: CK_C_InitToken,
    pub C_InitPIN: CK_C_InitPIN,
    pub C_SetPIN: CK_C_SetPIN,
    pub C_OpenSession: CK_C_OpenSession,
    pub C_CloseSession: CK_C_CloseSession,
    pub C_CloseAllSessions: CK_C_CloseAllSessions,
    pub C_GetSessionInfo: CK_C_GetSessionInfo,
    pub C_GetOperationState: CK_C_GetOperationState,
    pub C_SetOperationState: CK_C_SetOperationState,
    pub C_Login: CK_C_Login,
    pub C_Logout: CK_C_Logout,
    pub C_CreateObject: CK_C_CreateObject,
    pub C_CopyObject: CK_C_CopyObject,
    pub C_DestroyObject: CK_C_DestroyObject,
    pub C_GetObjectSize: CK_C_GetObjectSize,
    pub C_GetAttributeValue: CK_C_GetAttributeValue,
    pub C_SetAttributeValue: CK_C_SetAttributeValue,
    pub C_FindObjectsInit: CK_C_FindObjectsInit,
    pub C_FindObjects: CK_C_FindObjects,
    pub C_FindObjectsFinal: CK_C_FindObjectsFinal,
    pub C_EncryptInit: CK_C_EncryptInit,
    pub C_Encrypt: CK_C_Encrypt,
    pub C_EncryptUpdate: CK_C_EncryptUpdate,
    pub C_EncryptFinal: CK_C_EncryptFinal,
    pub C_DecryptInit: CK_C_DecryptInit,
    pub C_Decrypt: CK_C_Decrypt,
    pub C_DecryptUpdate: CK_C_DecryptUpdate,
    pub C_DecryptFinal: CK_C_DecryptFinal,
    pub C_DigestInit: CK_C_DigestInit,
    pub C_Digest: CK_C_Digest,
    pub C_DigestUpdate: CK_C_DigestUpdate,
    pub C_DigestKey: CK_C_DigestKey,
    pub C_DigestFinal: CK_C_DigestFinal,
    pub C_SignInit: CK_C_SignInit,
    pub C_Sign: CK_C_Sign,
    pub C_SignUpdate: CK_C_SignUpdate,
    pub C_SignFinal: CK_C_SignFinal,
    pub C_SignRecoverInit: CK_C_SignRecoverInit,
    pub C_SignRecover: CK_C_SignRecover,
    pub C_VerifyInit: CK_C_VerifyInit,
    pub C_Verify: CK_C_Verify,
    pub C_VerifyUpdate: CK_C_VerifyUpdate,
    pub C_VerifyFinal: CK_C_VerifyFinal,
    pub C_VerifyRecoverInit: CK_C_VerifyRecoverInit,
    pub C_VerifyRecover: CK_C_VerifyRecover,
    pub C_DigestEncryptUpdate: CK_C_DigestEncryptUpdate,
    pub C_DecryptDigestUpdate: CK_C_DecryptDigestUpdate,
    pub C_SignEncryptUpdate: CK_C_SignEncryptUpdate,
    pub C_DecryptVerifyUpdate: CK_C_DecryptVerifyUpdate,
    pub C_GenerateKey: CK_C_GenerateKey,
    pub C_GenerateKeyPair: CK_C_GenerateKeyPair,
    pub C_WrapKey: CK_C_WrapKey,
    pub C_UnwrapKey: CK_C_UnwrapKey,
    pub C_DeriveKey: CK_C_DeriveKey,
    pub C_SeedRandom: CK_C_SeedRandom,
    pub C_GenerateRandom: CK_C_GenerateRandom,
    pub C_GetFunctionStatus: CK_C_GetFunctionStatus,
    pub C_CancelFunction: CK_C_CancelFunction,
    pub C_WaitForSlotEvent: CK_C_WaitForSlotEvent,
}
pub type CK_CREATEMUTEX = ::std::option::Option<
    unsafe extern "C" fn(mutex: *mut *mut ::std::os::raw::c_void) -> CK_RV,
>;
pub type CK_DESTROYMUTEX = ::std::option::Option<
    unsafe extern "C" fn(mutex: *mut ::std::os::raw::c_void) -> CK_RV,
>;
pub type CK_LOCKMUTEX = ::std::option::Option<
    unsafe extern "C" fn(mutex: *mut ::std::os::raw::c_void) -> CK_RV,
>;
pub type CK_UNLOCKMUTEX = ::std::option::Option<
    unsafe extern "C" fn(mutex: *mut ::std::os::raw::c_void) -> CK_RV,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CK_C_INITIALIZE_ARGS {
    pub CreateMutex: CK_CREATEMUTEX,
    pub DestroyMutex: CK_DESTROYMUTEX,
    pub LockMutex: CK_LOCKMUTEX,
    pub UnlockMutex: CK_UNLOCKMUTEX,
    pub flags: CK_FLAGS,
    pub pReserved: *mut ::std::os::raw::c_void,
}
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
pub type CK_BYTE = ::std::os::raw::c_uchar;
pub type CK_CHAR = ::std::os::raw::c_uchar;
pub type CK_UTF8CHAR = ::std::os::raw::c_uchar;
pub type CK_BBOOL = ::std::os::raw::c_uchar;
pub type CK_ULONG = ::std::os::raw::c_ulong;
pub type CK_LONG = ::std::os::raw::c_long;
pub type CK_BYTE_PTR = *mut CK_BYTE;
pub type CK_CHAR_PTR = *mut CK_CHAR;
pub type CK_UTF8CHAR_PTR = *mut CK_UTF8CHAR;
pub type CK_ULONG_PTR = *mut CK_ULONG;
pub type CK_VOID_PTR = *mut ::std::os::raw::c_void;
pub type CK_VOID_PTR_PTR = *mut *mut ::std::os::raw::c_void;
pub type CK_VERSION = _CK_VERSION;
pub type CK_VERSION_PTR = *mut _CK_VERSION;
pub type CK_INFO = _CK_INFO;
pub type CK_INFO_PTR = *mut _CK_INFO;
pub type CK_SLOT_ID_PTR = *mut CK_SLOT_ID;
pub type CK_SLOT_INFO = _CK_SLOT_INFO;
pub type CK_SLOT_INFO_PTR = *mut _CK_SLOT_INFO;
pub type CK_TOKEN_INFO = _CK_TOKEN_INFO;
pub type CK_TOKEN_INFO_PTR = *mut _CK_TOKEN_INFO;
pub type CK_SESSION_HANDLE_PTR = *mut CK_SESSION_HANDLE;
pub type CK_SESSION_INFO = _CK_SESSION_INFO;
pub type CK_SESSION_INFO_PTR = *mut _CK_SESSION_INFO;
pub type CK_OBJECT_HANDLE_PTR = *mut CK_OBJECT_HANDLE;
pub type CK_OBJECT_CLASS_PTR = *mut CK_OBJECT_CLASS;
pub type CK_ATTRIBUTE = _CK_ATTRIBUTE;
pub type CK_ATTRIBUTE_PTR = *mut _CK_ATTRIBUTE;
pub type CK_DATE = _CK_DATE;
pub type CK_DATE_PTR = *mut _CK_DATE;
pub type CK_MECHANISM_TYPE_PTR = *mut CK_MECHANISM_TYPE;
pub type CK_MECHANISM = _CK_MECHANISM;
pub type CK_MECHANISM_PTR = *mut _CK_MECHANISM;
pub type CK_MECHANISM_INFO = _CK_MECHANISM_INFO;
pub type CK_MECHANISM_INFO_PTR = *mut _CK_MECHANISM_INFO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_otp_mechanism_info {
    _unused: [u8; 0],
}
pub type CK_OTP_MECHANISM_INFO = ck_otp_mechanism_info;
pub type CK_OTP_MECHANISM_INFO_PTR = *mut ck_otp_mechanism_info;
pub type CK_FUNCTION_LIST = _CK_FUNCTION_LIST;
pub type CK_FUNCTION_LIST_PTR = *mut _CK_FUNCTION_LIST;
pub type CK_FUNCTION_LIST_PTR_PTR = *mut *mut _CK_FUNCTION_LIST;
pub type CK_C_INITIALIZE_ARGS = _CK_C_INITIALIZE_ARGS;
pub type CK_C_INITIALIZE_ARGS_PTR = *mut _CK_C_INITIALIZE_ARGS;
pub type CK_RSA_PKCS_PSS_PARAMS = ck_rsa_pkcs_pss_params;
pub type CK_RSA_PKCS_PSS_PARAMS_PTR = *mut ck_rsa_pkcs_pss_params;
pub type CK_RSA_PKCS_OAEP_PARAMS = ck_rsa_pkcs_oaep_params;
pub type CK_RSA_PKCS_OAEP_PARAMS_PTR = *mut ck_rsa_pkcs_oaep_params;
pub type CK_AES_CTR_PARAMS = ck_aes_ctr_params;
pub type CK_AES_CTR_PARAMS_PTR = *mut ck_aes_ctr_params;
pub type CK_GCM_PARAMS = ck_gcm_params;
pub type CK_GCM_PARAMS_PTR = *mut ck_gcm_params;
pub type CK_ECDH1_DERIVE_PARAMS = ck_ecdh1_derive_params;
pub type CK_ECDH1_DERIVE_PARAMS_PTR = *mut ck_ecdh1_derive_params;
pub type CK_KEY_DERIVATION_STRING_DATA = ck_key_derivation_string_data;
pub type CK_KEY_DERIVATION_STRING_DATA_PTR = *mut ck_key_derivation_string_data;
pub type CK_DES_CBC_ENCRYPT_DATA_PARAMS = ck_des_cbc_encrypt_data_params;
pub type CK_DES_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut ck_des_cbc_encrypt_data_params;
pub type CK_AES_CBC_ENCRYPT_DATA_PARAMS = ck_aes_cbc_encrypt_data_params;
pub type CK_AES_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut ck_aes_cbc_encrypt_data_params;
