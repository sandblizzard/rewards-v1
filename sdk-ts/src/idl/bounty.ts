/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bounty.json`.
 */
export type Bounty = {
  "address": "HYtMRnS1UxUTJtvisReiwGEYPSV5LCtQPrsVnXCVJUyi",
  "metadata": {
    "name": "bounty",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addBountyDenomination",
      "docs": [
        "add bounty denomination",
        "it"
      ],
      "discriminator": [
        57,
        70,
        243,
        255,
        122,
        24,
        57,
        244
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "docs": [
            "protocol config"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "mint",
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "docs": [
            "bounty denoination to be created"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "const",
                "value": [
                  68,
                  69,
                  78,
                  79,
                  77,
                  73,
                  78,
                  65,
                  84,
                  73,
                  79,
                  78
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "feeCollector",
          "docs": [
            "Fee collector is owned by the protocol and",
            "collects fees from the bounty"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "const",
                "value": [
                  70,
                  69,
                  69,
                  95,
                  67,
                  79,
                  76,
                  76,
                  69,
                  67,
                  84,
                  79,
                  82
                ]
              },
              {
                "kind": "account",
                "path": "mint"
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
      "name": "addRelayer",
      "discriminator": [
        184,
        240,
        94,
        199,
        19,
        71,
        21,
        192
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "relayer",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "arg",
                "path": "relayerAddress"
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
          "name": "relayerAddress",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "claimRewards",
      "docs": [
        "Claim rewards",
        "",
        "Claim rewards for the bounty"
      ],
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "solver",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "solver.owner",
                "account": "solver"
              }
            ]
          }
        },
        {
          "name": "solverTokenAccount",
          "docs": [
            "token pda"
          ],
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "completeBounty",
      "docs": [
        "completeBounty",
        "",
        "Try to complete bounty"
      ],
      "discriminator": [
        175,
        126,
        79,
        116,
        248,
        106,
        31,
        117
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "only owners or relayers can complete bounties"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "sandMint",
          "writable": true
        },
        {
          "name": "feeCollector",
          "writable": true
        },
        {
          "name": "bounty",
          "docs": [
            "bounty to be completed",
            "FIXME"
          ],
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty"
              }
            ]
          }
        },
        {
          "name": "solverTokenAccount1",
          "writable": true
        },
        {
          "name": "solverTokenAccount2",
          "writable": true,
          "optional": true
        },
        {
          "name": "solver1",
          "docs": [
            "up to 4 receivers"
          ],
          "writable": true
        },
        {
          "name": "solver2",
          "writable": true,
          "optional": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "completeBountyAsRelayer",
      "docs": [
        "completeBounty",
        "",
        "Try to complete bounty"
      ],
      "discriminator": [
        133,
        215,
        10,
        55,
        154,
        134,
        42,
        86
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "only owners or relayers can complete bounties"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "sandMint",
          "writable": true
        },
        {
          "name": "feeCollector",
          "writable": true
        },
        {
          "name": "bounty",
          "docs": [
            "bounty to be completed",
            "FIXME"
          ],
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty"
              }
            ]
          }
        },
        {
          "name": "solverTokenAccount1",
          "writable": true
        },
        {
          "name": "solverTokenAccount2",
          "writable": true,
          "optional": true
        },
        {
          "name": "solver1",
          "docs": [
            "up to 4 receivers"
          ],
          "writable": true
        },
        {
          "name": "solver2",
          "writable": true,
          "optional": true
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
          "name": "relayer",
          "docs": [
            "relayer that wants to complete the transaction",
            "validate the seeds"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "relayer.owner",
                "account": "relayer"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "createBounty",
      "docs": [
        "createBounty",
        "",
        "creates a bounty"
      ],
      "discriminator": [
        122,
        90,
        14,
        143,
        8,
        125,
        200,
        2
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "bounty",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "arg",
                "path": "id"
              }
            ]
          }
        },
        {
          "name": "domain",
          "docs": [
            "domain to attach the bounty to"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "domain.data.platform",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.organization",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.team",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.domain_type",
                "account": "domain"
              }
            ]
          }
        },
        {
          "name": "creatorAccount",
          "docs": [
            "Account to credit the user"
          ],
          "writable": true
        },
        {
          "name": "bountyDenomination",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "const",
                "value": [
                  68,
                  69,
                  78,
                  79,
                  77,
                  73,
                  78,
                  65,
                  84,
                  73,
                  79,
                  78
                ]
              },
              {
                "kind": "account",
                "path": "bounty_denomination.mint",
                "account": "denomination"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "docs": [
            "Bounty escrow to transfer funds to"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty"
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
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u64"
        },
        {
          "name": "externalId",
          "type": "string"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "endsAt",
          "type": {
            "option": "i64"
          }
        }
      ]
    },
    {
      "name": "createDomain",
      "docs": [
        "create domain"
      ],
      "discriminator": [
        103,
        208,
        151,
        155,
        64,
        18,
        133,
        109
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "domain",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "arg",
                "path": "platform"
              },
              {
                "kind": "arg",
                "path": "organization"
              },
              {
                "kind": "arg",
                "path": "team"
              },
              {
                "kind": "arg",
                "path": "domainType"
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
          "name": "domainType",
          "type": "string"
        },
        {
          "name": "platform",
          "type": "string"
        },
        {
          "name": "organization",
          "type": "string"
        },
        {
          "name": "team",
          "type": "string"
        },
        {
          "name": "installationId",
          "type": "u32"
        }
      ]
    },
    {
      "name": "deactivateBountyDenomination",
      "docs": [
        "deactivate bounty denomination"
      ],
      "discriminator": [
        223,
        116,
        71,
        26,
        98,
        198,
        235,
        104
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "docs": [
            "bounty denoination to be created"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "const",
                "value": [
                  68,
                  69,
                  78,
                  79,
                  77,
                  73,
                  78,
                  65,
                  84,
                  73,
                  79,
                  78
                ]
              },
              {
                "kind": "account",
                "path": "mint"
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
      "name": "deactivateDomain",
      "docs": [
        "deactivate domain"
      ],
      "discriminator": [
        135,
        190,
        180,
        160,
        230,
        33,
        245,
        185
      ],
      "accounts": [
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "domain",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "domain.data.platform",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.organization",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.team",
                "account": "domain"
              },
              {
                "kind": "account",
                "path": "domain.data.domain_type",
                "account": "domain"
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
      "name": "donateToBounty",
      "docs": [
        "donateToBounty"
      ],
      "discriminator": [
        112,
        242,
        50,
        166,
        86,
        201,
        169,
        91
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "bounty",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty.id_bytes",
                "account": "bounty"
              }
            ]
          }
        },
        {
          "name": "donaterTokenAccount",
          "docs": [
            "Account to credit the user"
          ],
          "writable": true
        },
        {
          "name": "escrow",
          "docs": [
            "Bounty escrow to transfer funds to"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty"
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
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
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
      "name": "initialize",
      "docs": [
        "initialize",
        "",
        "- Initializes the protocol",
        "- creates the bounty mint"
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
          "name": "protocolOwner",
          "docs": [
            "creator is the owner of the protocol",
            "should become a smart wallet over time"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "protocol",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "sandMint",
          "docs": [
            "mint to be used to distribute rewards"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  97,
                  110,
                  100,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "docs": [
            "create treasury account to hold the protocol's funds"
          ],
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "rentSysvar",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "proposeBountySolution",
      "docs": [
        "proposeBountySolution"
      ],
      "discriminator": [
        44,
        26,
        76,
        164,
        169,
        42,
        64,
        203
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "bounty",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty.id_bytes",
                "account": "bounty"
              }
            ]
          }
        },
        {
          "name": "bountySolution",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "bounty"
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "solution",
          "type": "string"
        }
      ]
    },
    {
      "name": "registerSolver",
      "docs": [
        "register solver",
        "",
        "Register solver for the first time in the protocol",
        "This will create a new solver account and a token account"
      ],
      "discriminator": [
        143,
        125,
        182,
        215,
        172,
        69,
        137,
        105
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol"
        },
        {
          "name": "solverAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "sandMint",
          "writable": true
        },
        {
          "name": "solverTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "signer"
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
                "path": "sandMint"
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
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": []
    },
    {
      "name": "removeRelayer",
      "discriminator": [
        154,
        149,
        161,
        231,
        69,
        74,
        136,
        237
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "protocol",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              }
            ]
          }
        },
        {
          "name": "relayer",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  66,
                  79,
                  85,
                  78,
                  84,
                  89,
                  95,
                  83,
                  65,
                  78,
                  68,
                  66,
                  76,
                  73,
                  90,
                  90,
                  65,
                  82,
                  68
                ]
              },
              {
                "kind": "account",
                "path": "relayer.owner",
                "account": "relayer"
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
    }
  ],
  "accounts": [
    {
      "name": "bounty",
      "discriminator": [
        237,
        16,
        105,
        198,
        19,
        69,
        242,
        234
      ]
    },
    {
      "name": "bountySolution",
      "discriminator": [
        126,
        42,
        156,
        43,
        28,
        65,
        71,
        118
      ]
    },
    {
      "name": "denomination",
      "discriminator": [
        255,
        95,
        246,
        40,
        56,
        163,
        107,
        85
      ]
    },
    {
      "name": "domain",
      "discriminator": [
        167,
        191,
        231,
        63,
        146,
        41,
        115,
        27
      ]
    },
    {
      "name": "protocol",
      "discriminator": [
        45,
        39,
        101,
        43,
        115,
        72,
        131,
        40
      ]
    },
    {
      "name": "relayer",
      "discriminator": [
        168,
        116,
        52,
        174,
        161,
        196,
        71,
        218
      ]
    },
    {
      "name": "solver",
      "discriminator": [
        174,
        70,
        187,
        101,
        208,
        40,
        95,
        77
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "canNotReinitBounty",
      "msg": "bounty can not be reinitialized"
    },
    {
      "code": 6001,
      "name": "bountyIsCompleted",
      "msg": "bounty is completed"
    },
    {
      "code": 6002,
      "name": "notAuthToCompleteBounty",
      "msg": "signer missing auth to complete bounty"
    },
    {
      "code": 6003,
      "name": "notAuthToReleaseEscrow",
      "msg": "signer missing auth to release escrow"
    },
    {
      "code": 6004,
      "name": "missingReceiverTokenAccounts",
      "msg": "at least one receiver needs to be specified"
    },
    {
      "code": 6005,
      "name": "wrongFeeCollectorMint",
      "msg": "wrong mint for fee collector"
    },
    {
      "code": 6006,
      "name": "wrongProtocolFeeCollector",
      "msg": "fee collector does not match protocol fee collector"
    },
    {
      "code": 6007,
      "name": "wrongDenominationFeeCollector",
      "msg": "invalid denomination fee collector"
    },
    {
      "code": 6008,
      "name": "wrongDenominationMint",
      "msg": "invalid denomination mint"
    },
    {
      "code": 6009,
      "name": "accountIsNotSigner",
      "msg": "Account is not signer"
    },
    {
      "code": 6010,
      "name": "accountNotActive",
      "msg": "Account is not active"
    },
    {
      "code": 6011,
      "name": "domainNotActive",
      "msg": "Domain is not active"
    },
    {
      "code": 6012,
      "name": "noClaimableReward",
      "msg": "No claimable reward"
    },
    {
      "code": 6013,
      "name": "wrongProtocolMintAuthority",
      "msg": "Wrong protocol mint authority"
    },
    {
      "code": 6014,
      "name": "wrongSolverTokenAccountOwner",
      "msg": "Wrong solver token account owner"
    }
  ],
  "types": [
    {
      "name": "bounty",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Owner of bounty"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "state",
            "docs": [
              "State - created, closed"
            ],
            "type": {
              "defined": {
                "name": "bountyState"
              }
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bumpArray",
            "docs": [
              "for the seeds"
            ],
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "escrowBump",
            "type": "u8"
          },
          {
            "name": "escrow",
            "docs": [
              "escrow of the bounty"
            ],
            "type": "pubkey"
          },
          {
            "name": "domain",
            "docs": [
              "domain information"
            ],
            "type": "pubkey"
          },
          {
            "name": "idBytes",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "completedBy",
            "docs": [
              "WHo completed the bounty"
            ],
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "endsAt",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "externalId",
            "type": "string"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "donaters",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "donateAmount",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "solvers",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "solverSolutions",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "solvedBy",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "bountySolution",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "bumpArray",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "bounty",
            "type": "pubkey"
          },
          {
            "name": "solver",
            "type": "pubkey"
          },
          {
            "name": "solution",
            "type": "string"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "bountyState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "created"
          },
          {
            "name": "completed"
          }
        ]
      }
    },
    {
      "name": "denomination",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bumpArray",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "feeCollector",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "domain",
      "docs": [
        "Domain is the domain to be indexed",
        "ex: if the domain is github/sandblizzard/rewards_v1 then",
        "bounty_type = issues/pull_request",
        "platform: github",
        "owner: <user",
        "sub_domain: sandblizzard",
        "repo: rewards_v1"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "bump is used to sign transactions"
            ],
            "type": "u8"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "owner",
            "docs": [
              "owner of the domain, could be an individual or dao",
              "it's the user who manage the domain"
            ],
            "type": "pubkey"
          },
          {
            "name": "installationId",
            "type": "u32"
          },
          {
            "name": "data",
            "docs": [
              "FIXME: Rename"
            ],
            "type": {
              "defined": {
                "name": "domainData"
              }
            }
          }
        ]
      }
    },
    {
      "name": "domainData",
      "docs": [
        "DomainIdentifier"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "platform",
            "docs": [
              "platform is the domain such as GitHub"
            ],
            "type": "string"
          },
          {
            "name": "organization",
            "docs": [
              "organization is the identifier within the domain",
              "like sandblizzard",
              "FIXME: rename"
            ],
            "type": "string"
          },
          {
            "name": "team",
            "docs": [
              "team is the identifier within the domain",
              "like rewards_v1. This corresponds to the Bounty"
            ],
            "type": "string"
          },
          {
            "name": "domainType",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "protocol",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bumpSeed",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "sandMint",
            "type": "pubkey"
          },
          {
            "name": "emission",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "relayer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bumpSeed",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "active",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "solver",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bumpSeed",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "miningMint",
            "type": "pubkey"
          },
          {
            "name": "claimableRewards",
            "type": "u64"
          },
          {
            "name": "totalRewards",
            "type": "u64"
          },
          {
            "name": "totalSolvedBounties",
            "type": "u64"
          },
          {
            "name": "active",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
