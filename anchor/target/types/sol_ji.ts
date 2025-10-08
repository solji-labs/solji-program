/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/sol_ji.json`.
 */
export type SolJi = {
  "address": "EHeMFMeN22VbNmB9BnPaQLRFVVGRuw5uEU6bVSM3eWaw",
  "metadata": {
    "name": "solJi",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "coinFlip",
      "docs": [
        "随机数前置指令"
      ],
      "discriminator": [
        229,
        124,
        31,
        2,
        166,
        139,
        34,
        248
      ],
      "accounts": [
        {
          "name": "playerState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114,
                  83,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "randomnessAccountData"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createDonateCount",
      "docs": [
        "捐助计数器"
      ],
      "discriminator": [
        27,
        49,
        4,
        25,
        227,
        205,
        54,
        217
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "donateCount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  111,
                  110,
                  97,
                  116,
                  101,
                  95,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "featsNftMintAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  114,
                  101,
                  97,
                  116,
                  101,
                  95,
                  102,
                  101,
                  97,
                  116,
                  115,
                  95,
                  110,
                  102,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createDonateRecord",
      "docs": [
        "捐助"
      ],
      "discriminator": [
        171,
        49,
        129,
        41,
        22,
        149,
        213,
        89
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "donateCount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  111,
                  110,
                  97,
                  116,
                  101,
                  95,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "donateRecord",
          "writable": true
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "masterEditonAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "featsNftMintAccount"
              },
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "featsNftMintAccount"
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "featsNftMintAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  114,
                  101,
                  97,
                  116,
                  101,
                  95,
                  102,
                  101,
                  97,
                  116,
                  115,
                  95,
                  110,
                  102,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "userReceiveFeatsNftAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "userInfo"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "featsNftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createLike",
      "docs": [
        "点赞"
      ],
      "discriminator": [
        211,
        47,
        253,
        78,
        254,
        205,
        32,
        184
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "publishWish",
          "writable": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "wishLike",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  119,
                  105,
                  115,
                  104,
                  95,
                  108,
                  105,
                  107,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "userInfo"
              },
              {
                "kind": "account",
                "path": "publishWish"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createTemple",
      "docs": [
        "创建寺庙"
      ],
      "discriminator": [
        3,
        126,
        203,
        122,
        145,
        137,
        43,
        242
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createUser",
      "docs": [
        "创建用户烧香,抽签,许愿前调用一次创建用户信息"
      ],
      "discriminator": [
        108,
        227,
        130,
        130,
        252,
        109,
        75,
        218
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createWish",
      "docs": [
        "许愿"
      ],
      "discriminator": [
        47,
        64,
        159,
        45,
        95,
        19,
        61,
        165
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "publishWish",
          "writable": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "content",
          "type": "string"
        },
        {
          "name": "isAnonymous",
          "type": "bool"
        }
      ]
    },
    {
      "name": "destroy",
      "docs": [
        "销毁nft"
      ],
      "discriminator": [
        157,
        40,
        96,
        3,
        135,
        203,
        143,
        74
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "nftMintAccount",
          "writable": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "userReceiveNftAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "userInfo"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "drawLots",
      "docs": [
        "抽签"
      ],
      "discriminator": [
        48,
        248,
        141,
        95,
        142,
        32,
        235,
        103
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "lotteryArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  111,
                  116,
                  116,
                  101,
                  114,
                  121,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "lotteryRecord",
          "writable": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "playerState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114,
                  83,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "randomnessAccountData"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "incenseBurn",
      "docs": [
        "烧香"
      ],
      "discriminator": [
        162,
        182,
        22,
        11,
        228,
        23,
        220,
        174
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "incenseRulesConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  99,
                  101,
                  110,
                  115,
                  101,
                  95,
                  114,
                  117,
                  108,
                  101,
                  115,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "masterEditonAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              },
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "nftMintAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  114,
                  101,
                  97,
                  116,
                  101,
                  95,
                  98,
                  117,
                  114,
                  110,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "args.name"
              }
            ]
          }
        },
        {
          "name": "userReceiveNftAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "userInfo"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "incenseBurnArgs"
            }
          }
        }
      ]
    },
    {
      "name": "incenseBuy",
      "docs": [
        "购买香"
      ],
      "discriminator": [
        65,
        182,
        91,
        20,
        158,
        168,
        80,
        18
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "incenseRulesConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  99,
                  101,
                  110,
                  115,
                  101,
                  95,
                  114,
                  117,
                  108,
                  101,
                  115,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "incenseType",
          "type": {
            "defined": {
              "name": "incenseType"
            }
          }
        },
        {
          "name": "number",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "初始化香配置"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "incenseRulesConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  99,
                  101,
                  110,
                  115,
                  101,
                  95,
                  114,
                  117,
                  108,
                  101,
                  115,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeLotteryPoetry",
      "docs": [
        "初始化签文"
      ],
      "discriminator": [
        5,
        192,
        155,
        65,
        57,
        146,
        216,
        82
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "lotteryArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  111,
                  116,
                  116,
                  101,
                  114,
                  121,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "playerState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114,
                  83,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "mintSbtNft",
      "docs": [
        "sbt nft"
      ],
      "discriminator": [
        211,
        221,
        122,
        86,
        196,
        17,
        189,
        171
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "sbtNftMintAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  114,
                  101,
                  97,
                  116,
                  101,
                  95,
                  115,
                  98,
                  116,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "args.name"
              }
            ]
          }
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "sbtNftMintAccount"
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "masterEditonAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "sbtNftMintAccount"
              },
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "sbtNftAssociatedTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "sbtNftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "sbtNftCount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  98,
                  116,
                  95,
                  110,
                  102,
                  116,
                  95,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "userInfo",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  105,
                  110,
                  102,
                  111
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createNftArgs"
            }
          }
        }
      ]
    },
    {
      "name": "nftMint",
      "docs": [
        "废弃"
      ],
      "discriminator": [
        114,
        240,
        84,
        69,
        251,
        232,
        33,
        209
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "nftMintAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  114,
                  101,
                  97,
                  116,
                  101,
                  95,
                  98,
                  117,
                  114,
                  110,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "args.name"
              }
            ]
          }
        },
        {
          "name": "masterEditonAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              },
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "metadataAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "nftAssociatedTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createNftArgs"
            }
          }
        }
      ]
    },
    {
      "name": "updateIncense",
      "docs": [
        "管理员修改规则"
      ],
      "discriminator": [
        192,
        24,
        250,
        194,
        1,
        204,
        231,
        30
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "incenseRulesConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  99,
                  101,
                  110,
                  115,
                  101,
                  95,
                  114,
                  117,
                  108,
                  101,
                  115,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "a",
          "type": {
            "defined": {
              "name": "incenseType"
            }
          }
        },
        {
          "name": "b",
          "type": {
            "defined": {
              "name": "incenseRule"
            }
          }
        }
      ]
    },
    {
      "name": "withdraw",
      "docs": [
        "temple 提现"
      ],
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "temple"
          ]
        },
        {
          "name": "temple",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "lamports",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "donateCounter",
      "discriminator": [
        171,
        145,
        150,
        142,
        109,
        29,
        184,
        37
      ]
    },
    {
      "name": "donateRecord",
      "discriminator": [
        51,
        16,
        75,
        204,
        27,
        186,
        177,
        80
      ]
    },
    {
      "name": "incenseRulesConfig",
      "discriminator": [
        200,
        195,
        32,
        169,
        180,
        113,
        201,
        138
      ]
    },
    {
      "name": "lotteryConfig",
      "discriminator": [
        174,
        54,
        184,
        175,
        81,
        20,
        237,
        24
      ]
    },
    {
      "name": "lotteryRecord",
      "discriminator": [
        160,
        163,
        139,
        153,
        174,
        152,
        206,
        198
      ]
    },
    {
      "name": "playerState",
      "discriminator": [
        56,
        3,
        60,
        86,
        174,
        16,
        244,
        195
      ]
    },
    {
      "name": "publishWish",
      "discriminator": [
        237,
        247,
        5,
        229,
        190,
        67,
        201,
        232
      ]
    },
    {
      "name": "sbtNftCount",
      "discriminator": [
        99,
        154,
        183,
        152,
        253,
        154,
        165,
        88
      ]
    },
    {
      "name": "temple",
      "discriminator": [
        138,
        212,
        122,
        18,
        215,
        198,
        211,
        9
      ]
    },
    {
      "name": "userInfo",
      "discriminator": [
        83,
        134,
        200,
        56,
        144,
        56,
        10,
        62
      ]
    },
    {
      "name": "wishLike",
      "discriminator": [
        225,
        166,
        172,
        94,
        61,
        243,
        114,
        238
      ]
    }
  ],
  "events": [
    {
      "name": "coinFlipEvent",
      "discriminator": [
        163,
        85,
        162,
        64,
        139,
        127,
        81,
        117
      ]
    },
    {
      "name": "destroyEvent",
      "discriminator": [
        126,
        26,
        251,
        130,
        144,
        54,
        247,
        156
      ]
    },
    {
      "name": "donateCountCreatedEvent",
      "discriminator": [
        214,
        175,
        204,
        229,
        86,
        149,
        161,
        68
      ]
    },
    {
      "name": "donateEvent",
      "discriminator": [
        108,
        195,
        128,
        89,
        65,
        3,
        166,
        91
      ]
    },
    {
      "name": "drawLotsEvent",
      "discriminator": [
        123,
        77,
        25,
        33,
        114,
        175,
        133,
        196
      ]
    },
    {
      "name": "incenseBoughtEvent",
      "discriminator": [
        4,
        134,
        188,
        182,
        136,
        160,
        174,
        44
      ]
    },
    {
      "name": "incenseBurnedEvent",
      "discriminator": [
        20,
        57,
        114,
        158,
        127,
        0,
        165,
        206
      ]
    },
    {
      "name": "likeCreatedEvent",
      "discriminator": [
        65,
        139,
        92,
        189,
        63,
        199,
        96,
        184
      ]
    },
    {
      "name": "medalMintedEvent",
      "discriminator": [
        191,
        201,
        207,
        61,
        30,
        134,
        204,
        157
      ]
    },
    {
      "name": "medalUpgradedEvent",
      "discriminator": [
        189,
        49,
        78,
        251,
        79,
        38,
        4,
        250
      ]
    },
    {
      "name": "sbtMintedEvent",
      "discriminator": [
        39,
        55,
        148,
        101,
        93,
        236,
        105,
        5
      ]
    },
    {
      "name": "templeWithdrawalEvent",
      "discriminator": [
        122,
        201,
        80,
        81,
        136,
        15,
        11,
        251
      ]
    },
    {
      "name": "wishCreatedEvent",
      "discriminator": [
        102,
        100,
        78,
        21,
        10,
        243,
        99,
        133
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "amountMustBeGreaterThanZero",
      "msg": "Withdraw amount must be greater than 0"
    },
    {
      "code": 6001,
      "name": "amountMustBeLessThanTempleBalance",
      "msg": "Withdraw amount exceeds available balance (after reserving rent-exempt minimum)."
    }
  ],
  "types": [
    {
      "name": "coinFlipEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "randomnessAccount",
            "type": "pubkey"
          },
          {
            "name": "commitSlot",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "createNftArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "collectionDetails",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "destroyEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "donateCountCreatedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "donateCounter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "count",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "donateEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "meritValue",
            "type": "u64"
          },
          {
            "name": "incenseValue",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "donateRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "meritValue",
            "type": "u64"
          },
          {
            "name": "incenseValue",
            "type": "u64"
          },
          {
            "name": "createAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "drawLotsEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "lotteryType",
            "type": {
              "defined": {
                "name": "lotteryType"
              }
            }
          },
          {
            "name": "meritChange",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "incenseBoughtEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "incenseType",
            "type": {
              "defined": {
                "name": "incenseType"
              }
            }
          },
          {
            "name": "number",
            "type": "u64"
          },
          {
            "name": "unitPrice",
            "type": "u64"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "incenseBurnArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "collectionDetails",
            "type": "bool"
          },
          {
            "name": "incenseType",
            "type": {
              "defined": {
                "name": "incenseType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "incenseBurnedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "incenseType",
            "type": {
              "defined": {
                "name": "incenseType"
              }
            }
          },
          {
            "name": "nftMint",
            "type": "pubkey"
          },
          {
            "name": "incenseValue",
            "type": "u64"
          },
          {
            "name": "meritValue",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "incenseRule",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "incensePrice",
            "type": "u64"
          },
          {
            "name": "meritValue",
            "type": "u64"
          },
          {
            "name": "incenseValue",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "incenseRulesConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "rules",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "incenseRule"
                  }
                },
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "incenseType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "faintScent"
          },
          {
            "name": "orangeIncense"
          },
          {
            "name": "ambergris"
          },
          {
            "name": "lingxiang"
          },
          {
            "name": "secretIncense"
          },
          {
            "name": "celestialIncense"
          }
        ]
      }
    },
    {
      "name": "likeCreatedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "liker",
            "type": "pubkey"
          },
          {
            "name": "wish",
            "type": "pubkey"
          },
          {
            "name": "newLikeCount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "lotteryConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lotteryArray",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "lotteryType"
                  }
                },
                7
              ]
            }
          }
        ]
      }
    },
    {
      "name": "lotteryRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "lotteryType",
            "type": {
              "defined": {
                "name": "lotteryType"
              }
            }
          },
          {
            "name": "createAt",
            "type": "i64"
          },
          {
            "name": "meritValue",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "lotteryType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "greatFortune"
          },
          {
            "name": "middleFortune"
          },
          {
            "name": "smallFortune"
          },
          {
            "name": "fortune"
          },
          {
            "name": "lateFortune"
          },
          {
            "name": "misfortune"
          },
          {
            "name": "greatMisfortune"
          }
        ]
      }
    },
    {
      "name": "medalLevel",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "bronze"
          },
          {
            "name": "silver"
          },
          {
            "name": "gold"
          },
          {
            "name": "supreme"
          }
        ]
      }
    },
    {
      "name": "medalMintedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "level",
            "type": "string"
          },
          {
            "name": "nftMint",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "medalUpgradedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "oldLevel",
            "type": "string"
          },
          {
            "name": "newLevel",
            "type": "string"
          },
          {
            "name": "nftMint",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "playerState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "allowedUser",
            "type": "pubkey"
          },
          {
            "name": "latestFlipResult",
            "type": "bool"
          },
          {
            "name": "randomnessAccount",
            "type": "pubkey"
          },
          {
            "name": "commitSlot",
            "type": "u64"
          },
          {
            "name": "settled",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "publishWish",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "content",
            "type": "string"
          },
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "createTime",
            "type": "i64"
          },
          {
            "name": "likeCount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "sbtMintedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "ata",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "url",
            "type": "string"
          },
          {
            "name": "donateAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "sbtNftCount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "temple",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "totalIncenseValue",
            "type": "u64"
          },
          {
            "name": "totalMeritValue",
            "type": "u64"
          },
          {
            "name": "totalLotteryCount",
            "type": "u64"
          },
          {
            "name": "totalWishCount",
            "type": "u64"
          },
          {
            "name": "totalDonateAmount",
            "type": "u64"
          },
          {
            "name": "buddhaNftCount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "templeWithdrawalEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "remainingBalance",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "userInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "burnCount",
            "type": {
              "array": [
                "u32",
                6
              ]
            }
          },
          {
            "name": "incensePropertyCount",
            "type": {
              "array": [
                "u32",
                6
              ]
            }
          },
          {
            "name": "meritValue",
            "type": "u64"
          },
          {
            "name": "incenseValue",
            "type": "u64"
          },
          {
            "name": "incenseTime",
            "type": "i64"
          },
          {
            "name": "donateAmount",
            "type": "u64"
          },
          {
            "name": "donateMeritValue",
            "type": "u64"
          },
          {
            "name": "donateIncenseValue",
            "type": "u64"
          },
          {
            "name": "currentMedalLevel",
            "type": {
              "option": {
                "defined": {
                  "name": "medalLevel"
                }
              }
            }
          },
          {
            "name": "lotteryCount",
            "type": "u32"
          },
          {
            "name": "lotteryIsFree",
            "type": "bool"
          },
          {
            "name": "lotteryTime",
            "type": "i64"
          },
          {
            "name": "wishTotalCount",
            "type": "u32"
          },
          {
            "name": "wishUpdateTime",
            "type": "i64"
          },
          {
            "name": "wishDailyCount",
            "type": "u32"
          },
          {
            "name": "hasSbtToken",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "wishCreatedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "content",
            "type": "string"
          },
          {
            "name": "value",
            "type": "u8"
          },
          {
            "name": "isAnonymous",
            "type": "bool"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "wishLike",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "likePubkey",
            "type": "pubkey"
          },
          {
            "name": "withPubkey",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
