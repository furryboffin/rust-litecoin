//! serde regression tests for blockdata::opcodes module.
//!

#![cfg(feature = "serde")]

extern crate litecoin;
extern crate serde_json;

macro_rules! test_opcodes {
    ($($op:ident),* $(,)+) => {
        $(
            let op = litecoin::blockdata::opcodes::all::$op;
            let want = concat!("\"", stringify!($op), "\"");
            let got = ::serde_json::to_string(&op).unwrap();
            assert_eq!(got, want);
        )*
    }
}

#[test]
fn serde_regression_opcodes() {
    test_opcodes! {
        OP_PUSHBYTES_0,
        OP_PUSHBYTES_1,
        OP_PUSHBYTES_2,
        OP_PUSHBYTES_3,
        OP_PUSHBYTES_4,
        OP_PUSHBYTES_5,
        OP_PUSHBYTES_6,
        OP_PUSHBYTES_7,
        OP_PUSHBYTES_8,
        OP_PUSHBYTES_9,
        OP_PUSHBYTES_10,
        OP_PUSHBYTES_11,
        OP_PUSHBYTES_12,
        OP_PUSHBYTES_13,
        OP_PUSHBYTES_14,
        OP_PUSHBYTES_15,
        OP_PUSHBYTES_16,
        OP_PUSHBYTES_17,
        OP_PUSHBYTES_18,
        OP_PUSHBYTES_19,
        OP_PUSHBYTES_20,
        OP_PUSHBYTES_21,
        OP_PUSHBYTES_22,
        OP_PUSHBYTES_23,
        OP_PUSHBYTES_24,
        OP_PUSHBYTES_25,
        OP_PUSHBYTES_26,
        OP_PUSHBYTES_27,
        OP_PUSHBYTES_28,
        OP_PUSHBYTES_29,
        OP_PUSHBYTES_30,
        OP_PUSHBYTES_31,
        OP_PUSHBYTES_32,
        OP_PUSHBYTES_33,
        OP_PUSHBYTES_34,
        OP_PUSHBYTES_35,
        OP_PUSHBYTES_36,
        OP_PUSHBYTES_37,
        OP_PUSHBYTES_38,
        OP_PUSHBYTES_39,
        OP_PUSHBYTES_40,
        OP_PUSHBYTES_41,
        OP_PUSHBYTES_42,
        OP_PUSHBYTES_43,
        OP_PUSHBYTES_44,
        OP_PUSHBYTES_45,
        OP_PUSHBYTES_46,
        OP_PUSHBYTES_47,
        OP_PUSHBYTES_48,
        OP_PUSHBYTES_49,
        OP_PUSHBYTES_50,
        OP_PUSHBYTES_51,
        OP_PUSHBYTES_52,
        OP_PUSHBYTES_53,
        OP_PUSHBYTES_54,
        OP_PUSHBYTES_55,
        OP_PUSHBYTES_56,
        OP_PUSHBYTES_57,
        OP_PUSHBYTES_58,
        OP_PUSHBYTES_59,
        OP_PUSHBYTES_60,
        OP_PUSHBYTES_61,
        OP_PUSHBYTES_62,
        OP_PUSHBYTES_63,
        OP_PUSHBYTES_64,
        OP_PUSHBYTES_65,
        OP_PUSHBYTES_66,
        OP_PUSHBYTES_67,
        OP_PUSHBYTES_68,
        OP_PUSHBYTES_69,
        OP_PUSHBYTES_70,
        OP_PUSHBYTES_71,
        OP_PUSHBYTES_72,
        OP_PUSHBYTES_73,
        OP_PUSHBYTES_74,
        OP_PUSHBYTES_75,
        OP_PUSHDATA1,
        OP_PUSHDATA2,
        OP_PUSHDATA4,
        OP_PUSHNUM_NEG1,
        OP_RESERVED,
        OP_PUSHNUM_1,
        OP_PUSHNUM_2,
        OP_PUSHNUM_3,
        OP_PUSHNUM_4,
        OP_PUSHNUM_5,
        OP_PUSHNUM_6,
        OP_PUSHNUM_7,
        OP_PUSHNUM_8,
        OP_PUSHNUM_9,
        OP_PUSHNUM_10,
        OP_PUSHNUM_11,
        OP_PUSHNUM_12,
        OP_PUSHNUM_13,
        OP_PUSHNUM_14,
        OP_PUSHNUM_15,
        OP_PUSHNUM_16,
        OP_NOP,
        OP_VER,
        OP_IF,
        OP_NOTIF,
        OP_VERIF,
        OP_VERNOTIF,
        OP_ELSE,
        OP_ENDIF,
        OP_VERIFY,
        OP_RETURN,
        OP_TOALTSTACK,
        OP_FROMALTSTACK,
        OP_2DROP,
        OP_2DUP,
        OP_3DUP,
        OP_2OVER,
        OP_2ROT,
        OP_2SWAP,
        OP_IFDUP,
        OP_DEPTH,
        OP_DROP,
        OP_DUP,
        OP_NIP,
        OP_OVER,
        OP_PICK,
        OP_ROLL,
        OP_ROT,
        OP_SWAP,
        OP_TUCK,
        OP_CAT,
        OP_SUBSTR,
        OP_LEFT,
        OP_RIGHT,
        OP_SIZE,
        OP_INVERT,
        OP_AND,
        OP_OR,
        OP_XOR,
        OP_EQUAL,
        OP_EQUALVERIFY,
        OP_RESERVED1,
        OP_RESERVED2,
        OP_1ADD,
        OP_1SUB,
        OP_2MUL,
        OP_2DIV,
        OP_NEGATE,
        OP_ABS,
        OP_NOT,
        OP_0NOTEQUAL,
        OP_ADD,
        OP_SUB,
        OP_MUL,
        OP_DIV,
        OP_MOD,
        OP_LSHIFT,
        OP_RSHIFT,
        OP_BOOLAND,
        OP_BOOLOR,
        OP_NUMEQUAL,
        OP_NUMEQUALVERIFY,
        OP_NUMNOTEQUAL,
        OP_LESSTHAN ,
        OP_GREATERTHAN ,
        OP_LESSTHANOREQUAL ,
        OP_GREATERTHANOREQUAL ,
        OP_MIN,
        OP_MAX,
        OP_WITHIN,
        OP_RIPEMD160,
        OP_SHA1,
        OP_SHA256,
        OP_HASH160,
        OP_HASH256,
        OP_CODESEPARATOR,
        OP_CHECKSIG,
        OP_CHECKSIGVERIFY,
        OP_CHECKMULTISIG,
        OP_CHECKMULTISIGVERIFY,
        OP_NOP1,
        OP_CLTV,
        OP_CSV,
        OP_NOP4,
        OP_NOP5,
        OP_NOP6,
        OP_NOP7,
        OP_NOP8,
        OP_NOP9,
        OP_NOP10,
        OP_CHECKSIGADD,
        OP_RETURN_187,
        OP_RETURN_188,
        OP_RETURN_189,
        OP_RETURN_190,
        OP_RETURN_191,
        OP_RETURN_192,
        OP_RETURN_193,
        OP_RETURN_194,
        OP_RETURN_195,
        OP_RETURN_196,
        OP_RETURN_197,
        OP_RETURN_198,
        OP_RETURN_199,
        OP_RETURN_200,
        OP_RETURN_201,
        OP_RETURN_202,
        OP_RETURN_203,
        OP_RETURN_204,
        OP_RETURN_205,
        OP_RETURN_206,
        OP_RETURN_207,
        OP_RETURN_208,
        OP_RETURN_209,
        OP_RETURN_210,
        OP_RETURN_211,
        OP_RETURN_212,
        OP_RETURN_213,
        OP_RETURN_214,
        OP_RETURN_215,
        OP_RETURN_216,
        OP_RETURN_217,
        OP_RETURN_218,
        OP_RETURN_219,
        OP_RETURN_220,
        OP_RETURN_221,
        OP_RETURN_222,
        OP_RETURN_223,
        OP_RETURN_224,
        OP_RETURN_225,
        OP_RETURN_226,
        OP_RETURN_227,
        OP_RETURN_228,
        OP_RETURN_229,
        OP_RETURN_230,
        OP_RETURN_231,
        OP_RETURN_232,
        OP_RETURN_233,
        OP_RETURN_234,
        OP_RETURN_235,
        OP_RETURN_236,
        OP_RETURN_237,
        OP_RETURN_238,
        OP_RETURN_239,
        OP_RETURN_240,
        OP_RETURN_241,
        OP_RETURN_242,
        OP_RETURN_243,
        OP_RETURN_244,
        OP_RETURN_245,
        OP_RETURN_246,
        OP_RETURN_247,
        OP_RETURN_248,
        OP_RETURN_249,
        OP_RETURN_250,
        OP_RETURN_251,
        OP_RETURN_252,
        OP_RETURN_253,
        OP_RETURN_254,
        OP_INVALIDOPCODE,
    }
}
