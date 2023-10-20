export const IDL = {
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
                    "type": "string"
                },
                {
                    "name": "bountyAmount",
                    "type": "u64"
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
                    "name": "feeCollector",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "bountyDenomination",
                    "isMut": false,
                    "isSigner": false,
                    "docs": [
                        "bounty denomination is the allowed denomination of a bounty",
                        "it needs to be checked against the fee collector and the mint"
                    ]
                },
                {
                    "name": "relayer",
                    "isMut": false,
                    "isSigner": false,
                    "docs": [
                        "relayer that wants to complete the transaction",
                        "validate the seeds"
                    ]
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
                    "name": "solver3",
                    "isMut": true,
                    "isSigner": false,
                    "isOptional": true
                },
                {
                    "name": "solver4",
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
        }
    ],
    "accounts": [
        {
            "name": "bounty",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "id",
                        "type": "string"
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
                        "name": "domainBytes",
                        "docs": [
                            "domain as bytes"
                        ],
                        "type": "bytes"
                    },
                    {
                        "name": "bountyAmount",
                        "type": "u64"
                    },
                    {
                        "name": "completedBy",
                        "type": {
                            "option": "publicKey"
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
            "name": "NotAuthToCompleteBounty",
            "msg": "signer missing auth to complete bounty"
        },
        {
            "code": 6002,
            "name": "NotAuthToReleaseEscrow",
            "msg": "signer missing auth to release escrow"
        },
        {
            "code": 6003,
            "name": "MissingReceiverTokenAccounts",
            "msg": "at least one receiver needs to be specified"
        },
        {
            "code": 6004,
            "name": "WrongFeeCollectorMint",
            "msg": "wrong mint for fee collector"
        },
        {
            "code": 6005,
            "name": "WrongProtocolFeeCollector",
            "msg": "fee collector does not match protocol fee collector"
        },
        {
            "code": 6006,
            "name": "WrongDenominationFeeCollector",
            "msg": "invalid denomination fee collector"
        },
        {
            "code": 6007,
            "name": "WrongDenominationMint",
            "msg": "invalid denomination mint"
        }
    ]
};
//# sourceMappingURL=bounty.js.map