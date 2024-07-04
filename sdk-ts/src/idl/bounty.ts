export type Bounty = {
  "version": "0.1.0",
  "name": "bounty",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "initialize",
        "",
        "- Initializes the protocol",
        "- creates the bounty mint"
      ],
      "accounts": [
        {
          "name": "protocolOwner",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "creator is the owner of the protocol",
            "should become a smart wallet over time"
          ]
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "protocol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "mint to be used to distribute rewards"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "create treasury account to hold the protocol's funds"
          ]
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rentSysvar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "registerSolver",
      "docs": [
        "register solver",
        "",
        "Register solver for the first time in the protocol",
        "This will create a new solver account and a token account"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solverAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addBountyDenomination",
      "docs": [
        "add bounty denomination",
        "it"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "protocol config"
          ]
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty denoination to be created"
          ]
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Fee collector is owned by the protocol and",
            "collects fees from the bounty"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deactivateBountyDenomination",
      "docs": [
        "deactivate bounty denomination"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty denoination to be created"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createBounty",
      "docs": [
        "create_bounty",
        "",
        "creates a bounty"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "domain",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "domain to attach the bounty to"
          ]
        },
        {
          "name": "creatorAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Account to credit the user"
          ]
        },
        {
          "name": "bountyDenomination",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bounty escrow to transfer funds to"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "donateToBounty",
      "docs": [
        "donate_to_bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "donaterTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Account to credit the user"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bounty escrow to transfer funds to"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "proposeBountySolution",
      "docs": [
        "propose_bounty_solution"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bountySolution",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "completeBounty",
      "docs": [
        "complete_bounty",
        "",
        "Try to complete bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "only owners or relayers can complete bounties"
          ]
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty to be completed",
            "FIXME"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount1",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "solver1",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "up to 4 receivers"
          ]
        },
        {
          "name": "solver2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeBountyAsRelayer",
      "docs": [
        "complete_bounty",
        "",
        "Try to complete bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "only owners or relayers can complete bounties"
          ]
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty to be completed",
            "FIXME"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount1",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "solver1",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "up to 4 receivers"
          ]
        },
        {
          "name": "solver2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "relayer that wants to complete the transaction",
            "validate the seeds"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "addRelayer",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "relayerAddress",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "removeRelayer",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createDomain",
      "docs": [
        "create domain"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "domain",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
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
        }
      ]
    },
    {
      "name": "deactivateDomain",
      "docs": [
        "deactivate domain"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "domain",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimRewards",
      "docs": [
        "Claim rewards",
        "",
        "Claim rewards for the bounty"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "token pda"
          ]
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
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
            "type": "publicKey"
          },
          {
            "name": "solver",
            "type": "publicKey"
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
      "name": "bounty",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Owner of bounty"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "state",
            "docs": [
              "State - created, closed"
            ],
            "type": {
              "defined": "BountyState"
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
            "type": "publicKey"
          },
          {
            "name": "domain",
            "docs": [
              "domain information"
            ],
            "type": "publicKey"
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
              "option": "publicKey"
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
              "vec": "publicKey"
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
              "vec": "publicKey"
            }
          },
          {
            "name": "solverSolutions",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "solvedBy",
            "type": {
              "vec": "publicKey"
            }
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
            "type": "publicKey"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "feeCollector",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "data",
            "docs": [
              "FIXME: Rename"
            ],
            "type": {
              "defined": "DomainData"
            }
          }
        ]
      }
    },
    {
      "name": "protocolCollector",
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
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "sandMint",
            "type": "publicKey"
          },
          {
            "name": "claimableSand",
            "type": "u64"
          },
          {
            "name": "claimableFee",
            "type": "u64"
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
            "type": "publicKey"
          },
          {
            "name": "sandMint",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "active",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "relayers",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "relayers",
            "type": {
              "vec": "publicKey"
            }
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
            "type": "publicKey"
          },
          {
            "name": "miningMint",
            "type": "publicKey"
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
  ],
  "types": [
    {
      "name": "DomainData",
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
      "name": "BountyState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Created"
          },
          {
            "name": "Completed"
          }
        ]
      }
    },
    {
      "name": "AnySolver",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "ProtocolCollector"
          },
          {
            "name": "Solver"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "CanNotReinitBounty",
      "msg": "bounty can not be reinitialized"
    },
    {
      "code": 6001,
      "name": "BountyIsCompleted",
      "msg": "bounty is completed"
    },
    {
      "code": 6002,
      "name": "NotAuthToCompleteBounty",
      "msg": "signer missing auth to complete bounty"
    },
    {
      "code": 6003,
      "name": "NotAuthToReleaseEscrow",
      "msg": "signer missing auth to release escrow"
    },
    {
      "code": 6004,
      "name": "MissingReceiverTokenAccounts",
      "msg": "at least one receiver needs to be specified"
    },
    {
      "code": 6005,
      "name": "WrongFeeCollectorMint",
      "msg": "wrong mint for fee collector"
    },
    {
      "code": 6006,
      "name": "WrongProtocolFeeCollector",
      "msg": "fee collector does not match protocol fee collector"
    },
    {
      "code": 6007,
      "name": "WrongDenominationFeeCollector",
      "msg": "invalid denomination fee collector"
    },
    {
      "code": 6008,
      "name": "WrongDenominationMint",
      "msg": "invalid denomination mint"
    },
    {
      "code": 6009,
      "name": "AccountIsNotSigner",
      "msg": "Account is not signer"
    },
    {
      "code": 6010,
      "name": "AccountNotActive",
      "msg": "Account is not active"
    },
    {
      "code": 6011,
      "name": "DomainNotActive",
      "msg": "Domain is not active"
    },
    {
      "code": 6012,
      "name": "NoClaimableReward",
      "msg": "No claimable reward"
    },
    {
      "code": 6013,
      "name": "WrongProtocolMintAuthority",
      "msg": "Wrong protocol mint authority"
    },
    {
      "code": 6014,
      "name": "WrongSolverTokenAccountOwner",
      "msg": "Wrong solver token account owner"
    }
  ]
};

export const IDL: Bounty = {
  "version": "0.1.0",
  "name": "bounty",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "initialize",
        "",
        "- Initializes the protocol",
        "- creates the bounty mint"
      ],
      "accounts": [
        {
          "name": "protocolOwner",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "creator is the owner of the protocol",
            "should become a smart wallet over time"
          ]
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "protocol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "mint to be used to distribute rewards"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "create treasury account to hold the protocol's funds"
          ]
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rentSysvar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "registerSolver",
      "docs": [
        "register solver",
        "",
        "Register solver for the first time in the protocol",
        "This will create a new solver account and a token account"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solverAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addBountyDenomination",
      "docs": [
        "add bounty denomination",
        "it"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "protocol config"
          ]
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty denoination to be created"
          ]
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Fee collector is owned by the protocol and",
            "collects fees from the bounty"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deactivateBountyDenomination",
      "docs": [
        "deactivate bounty denomination"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "mint to be used for denomination"
          ]
        },
        {
          "name": "denomination",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty denoination to be created"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createBounty",
      "docs": [
        "create_bounty",
        "",
        "creates a bounty"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "domain",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "domain to attach the bounty to"
          ]
        },
        {
          "name": "creatorAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Account to credit the user"
          ]
        },
        {
          "name": "bountyDenomination",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bounty escrow to transfer funds to"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "donateToBounty",
      "docs": [
        "donate_to_bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "donaterTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Account to credit the user"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bounty escrow to transfer funds to"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "proposeBountySolution",
      "docs": [
        "propose_bounty_solution"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bountySolution",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "completeBounty",
      "docs": [
        "complete_bounty",
        "",
        "Try to complete bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "only owners or relayers can complete bounties"
          ]
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty to be completed",
            "FIXME"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount1",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "solver1",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "up to 4 receivers"
          ]
        },
        {
          "name": "solver2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeBountyAsRelayer",
      "docs": [
        "complete_bounty",
        "",
        "Try to complete bounty"
      ],
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "only owners or relayers can complete bounties"
          ]
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sandMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeCollector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bounty",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "bounty to be completed",
            "FIXME"
          ]
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount1",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "solver1",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "up to 4 receivers"
          ]
        },
        {
          "name": "solver2",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "relayer that wants to complete the transaction",
            "validate the seeds"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "addRelayer",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "relayerAddress",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "removeRelayer",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "relayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createDomain",
      "docs": [
        "create domain"
      ],
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "domain",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
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
        }
      ]
    },
    {
      "name": "deactivateDomain",
      "docs": [
        "deactivate domain"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "domain",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimRewards",
      "docs": [
        "Claim rewards",
        "",
        "Claim rewards for the bounty"
      ],
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "protocol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solver",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solverTokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "token pda"
          ]
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
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
            "type": "publicKey"
          },
          {
            "name": "solver",
            "type": "publicKey"
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
      "name": "bounty",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Owner of bounty"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "state",
            "docs": [
              "State - created, closed"
            ],
            "type": {
              "defined": "BountyState"
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
            "type": "publicKey"
          },
          {
            "name": "domain",
            "docs": [
              "domain information"
            ],
            "type": "publicKey"
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
              "option": "publicKey"
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
              "vec": "publicKey"
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
              "vec": "publicKey"
            }
          },
          {
            "name": "solverSolutions",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "solvedBy",
            "type": {
              "vec": "publicKey"
            }
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
            "type": "publicKey"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "feeCollector",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "data",
            "docs": [
              "FIXME: Rename"
            ],
            "type": {
              "defined": "DomainData"
            }
          }
        ]
      }
    },
    {
      "name": "protocolCollector",
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
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "sandMint",
            "type": "publicKey"
          },
          {
            "name": "claimableSand",
            "type": "u64"
          },
          {
            "name": "claimableFee",
            "type": "u64"
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
            "type": "publicKey"
          },
          {
            "name": "sandMint",
            "type": "publicKey"
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
            "type": "publicKey"
          },
          {
            "name": "active",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "relayers",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "relayers",
            "type": {
              "vec": "publicKey"
            }
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
            "type": "publicKey"
          },
          {
            "name": "miningMint",
            "type": "publicKey"
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
  ],
  "types": [
    {
      "name": "DomainData",
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
      "name": "BountyState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Created"
          },
          {
            "name": "Completed"
          }
        ]
      }
    },
    {
      "name": "AnySolver",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "ProtocolCollector"
          },
          {
            "name": "Solver"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "CanNotReinitBounty",
      "msg": "bounty can not be reinitialized"
    },
    {
      "code": 6001,
      "name": "BountyIsCompleted",
      "msg": "bounty is completed"
    },
    {
      "code": 6002,
      "name": "NotAuthToCompleteBounty",
      "msg": "signer missing auth to complete bounty"
    },
    {
      "code": 6003,
      "name": "NotAuthToReleaseEscrow",
      "msg": "signer missing auth to release escrow"
    },
    {
      "code": 6004,
      "name": "MissingReceiverTokenAccounts",
      "msg": "at least one receiver needs to be specified"
    },
    {
      "code": 6005,
      "name": "WrongFeeCollectorMint",
      "msg": "wrong mint for fee collector"
    },
    {
      "code": 6006,
      "name": "WrongProtocolFeeCollector",
      "msg": "fee collector does not match protocol fee collector"
    },
    {
      "code": 6007,
      "name": "WrongDenominationFeeCollector",
      "msg": "invalid denomination fee collector"
    },
    {
      "code": 6008,
      "name": "WrongDenominationMint",
      "msg": "invalid denomination mint"
    },
    {
      "code": 6009,
      "name": "AccountIsNotSigner",
      "msg": "Account is not signer"
    },
    {
      "code": 6010,
      "name": "AccountNotActive",
      "msg": "Account is not active"
    },
    {
      "code": 6011,
      "name": "DomainNotActive",
      "msg": "Domain is not active"
    },
    {
      "code": 6012,
      "name": "NoClaimableReward",
      "msg": "No claimable reward"
    },
    {
      "code": 6013,
      "name": "WrongProtocolMintAuthority",
      "msg": "Wrong protocol mint authority"
    },
    {
      "code": 6014,
      "name": "WrongSolverTokenAccountOwner",
      "msg": "Wrong solver token account owner"
    }
  ]
};
