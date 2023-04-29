use crate::{
    domains::utils::{get_key_from_env, SBError},
    load_keypair,
};

use anchor_client::{
    anchor_lang::{system_program, InstructionData, ToAccountMetas},
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, pubkey::*,
        signature::Keypair, signer::Signer,
    },
    *,
};
use anchor_spl::{token::TokenAccount, *};

use spl_associated_token_account::instruction::create_associated_token_account;

use bounty::{self, state::Bounty};
use spl_associated_token_account::get_associated_token_address;
/// Bounty is the SDK for the bounty program
use std::{rc::Rc, result::Result, str::FromStr, sync::Arc};

pub struct BountySdk {
    pub program: Program,
    pub cluster: Cluster,
    pub payer: Keypair,
}

pub fn get_bounty_connection() -> Result<(Program, Cluster), SBError> {
    let cluster_name = get_key_from_env("CLUSTER").unwrap();

    let payer = load_keypair().unwrap();
    let payer_rc = Rc::new(payer);
    let cluster = match Cluster::from_str(&cluster_name) {
        Ok(res) => res,
        Err(err) => {
            return Err(SBError::CouldNotGetEnvKey(
                "get_program_client".to_string(),
                "CLUSTER".to_string(),
                err.to_string(),
            ))
        }
    };

    let client = anchor_client::Client::new_with_options(
        cluster.clone(),
        payer_rc,
        CommitmentConfig::processed(),
    );
    let program = client.program(bounty::id());
    Ok((program, cluster))
}

pub fn get_bounty(
    domain: &str,
    sub_domain: &str,
    issue_id: &u64,
) -> Result<bounty::state::Bounty, SBError> {
    let (program, cluster) = get_bounty_connection()?;
    let bounty_pda = anchor_client::solana_sdk::pubkey::Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            domain.as_bytes(),
            sub_domain.as_bytes(),
            issue_id.to_string().as_bytes(),
        ],
        &bounty::ID,
    );

    let bounty = match program.account::<bounty::state::Bounty>(bounty_pda.0) {
        Ok(bounty) => bounty,
        Err(err) => {
            return Err(SBError::BountyDoesNotExistInState(
                bounty_pda.0.to_string(),
                err.to_string(),
            ))
        }
    };

    if bounty.id.eq("") {
        return Err(SBError::BountyDoesNotExistInState(
            bounty_pda.0.to_string(),
            format!(
            "Id of bounty with address {} on cluster={} by program_id={} is empty. Bounty: {:?}",
            bounty_pda.0,
            cluster.url(),
            bounty::id(),
            bounty
        ),
        ));
    }
    Ok(bounty)
}

impl BountySdk {
    pub fn new() -> Result<Arc<BountySdk>, SBError> {
        let cluster_name = get_key_from_env("CLUSTER").unwrap();

        let payer = load_keypair().unwrap();
        let payer_rc = Rc::new(payer);
        let cluster = match Cluster::from_str(&cluster_name) {
            Ok(res) => res,
            Err(err) => {
                return Err(SBError::CouldNotGetEnvKey(
                    "get_program_client".to_string(),
                    "CLUSTER".to_string(),
                    err.to_string(),
                ))
            }
        };

        let client = anchor_client::Client::new_with_options(
            cluster.clone(),
            payer_rc,
            CommitmentConfig::processed(),
        );
        let program = client.program(bounty::id());

        Ok(Arc::new(BountySdk {
            program,
            cluster,
            payer: load_keypair().unwrap(),
        }))
    }

    pub fn get_protocol(&self) -> Result<bounty::state::Protocol, SBError> {
        let protocol =
            Pubkey::find_program_address(&[bounty::utils::BOUNTY_SEED.as_bytes()], &bounty::id());
        let protocol = match self.program.account::<bounty::state::Protocol>(protocol.0) {
            Ok(bounty) => bounty,
            Err(err) => {
                return Err(SBError::BountyDoesNotExistInState(
                    protocol.0.to_string(),
                    err.to_string(),
                ))
            }
        };

        Ok(protocol)
    }

    pub fn get_escrow(&self, bounty: &Pubkey) -> Result<TokenAccount, SBError> {
        let protocol = Pubkey::find_program_address(&[bounty.to_bytes().as_ref()], &bounty::id());
        let escrow = match self.program.account::<TokenAccount>(protocol.0) {
            Ok(bounty) => bounty,
            Err(err) => {
                return Err(SBError::BountyDoesNotExistInState(
                    protocol.0.to_string(),
                    err.to_string(),
                ))
            }
        };

        Ok(escrow)
    }

    pub fn get_bounty(
        &self,
        domain: &str,
        sub_domain: &str,
        issue_id: &u64,
    ) -> Result<bounty::state::Bounty, SBError> {
        log::debug!(
            "[bounty_sdk] get_bounty for domain={} sub_domain={} issue_id={}",
            domain,
            sub_domain,
            issue_id
        );
        let bounty_pda = anchor_client::solana_sdk::pubkey::Pubkey::find_program_address(
            &[
                bounty::utils::BOUNTY_SEED.as_bytes(),
                domain.as_bytes(),
                sub_domain.as_bytes(),
                issue_id.to_string().as_bytes(),
            ],
            &bounty::ID,
        );

        let bounty = match self.program.account::<bounty::state::Bounty>(bounty_pda.0) {
            Ok(bounty) => bounty,
            Err(err) => {
                return Err(SBError::BountyDoesNotExistInState(
                    bounty_pda.0.to_string(),
                    err.to_string(),
                ))
            }
        };

        if bounty.id.eq("") {
            return Err(SBError::BountyDoesNotExistInState(
                bounty_pda.0.to_string(),
                format!(
                "Id of bounty with address {} on cluster={} by program_id={} is empty. Bounty: {:?}",
                bounty_pda.0,
                self.cluster.clone().url(),
                bounty::id(),
                bounty
            ),
            ));
        }
        Ok(bounty)
    }

    pub fn get_protocol_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[bounty::utils::BOUNTY_SEED.as_bytes()], &bounty::id())
    }

    pub fn get_relayer_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                bounty::utils::BOUNTY_SEED.as_bytes(),
                self.payer.pubkey().to_bytes().as_ref(),
            ],
            &bounty::id(),
        )
    }

    pub fn get_bounty_pda(&self, domain: &str, sub_domain: &str, issue_id: &u64) -> (Pubkey, u8) {
        anchor_client::solana_sdk::pubkey::Pubkey::find_program_address(
            &[
                bounty::utils::BOUNTY_SEED.as_bytes(),
                domain.as_bytes(),
                sub_domain.as_bytes(),
                issue_id.to_string().as_bytes(),
            ],
            &bounty::ID,
        )
    }

    pub fn get_escrow_pda(&self, bounty_pda: &Pubkey) -> (Pubkey, u8) {
        anchor_client::solana_sdk::pubkey::Pubkey::find_program_address(
            &[bounty_pda.to_bytes().as_ref()],
            &bounty::id(),
        )
    }

    pub fn get_fee_collector(&self, bounty_mint: &Pubkey) -> Pubkey {
        associated_token::get_associated_token_address(
            &Pubkey::from_str("CNY467c6XURCPjiXiKRLCvxdRf3bpunagYTJpr685gPv").unwrap(),
            bounty_mint,
        )
    }

    pub fn does_ata_exist(&self, owner: &Pubkey, mint: &Pubkey) -> bool {
        let account_address = get_associated_token_address(owner, mint);
        let account = match self.program.rpc().get_token_account(&account_address) {
            Ok(account) => account,
            Err(_err) => return false,
        };

        if account.is_some() {
            return true;
        };
        false
    }

    pub fn get_ata_instruction(&self, owner: &Pubkey, mint: &Pubkey) -> Instruction {
        create_associated_token_account(&self.payer.pubkey(), owner, mint, &anchor_spl::token::ID)
    }

    pub fn get_ata(&self, owners: &[Pubkey], token_mint: &Pubkey) -> Result<Vec<Pubkey>, SBError> {
        return Ok(owners
            .iter()
            .map(|owner| get_associated_token_address(owner, token_mint))
            .collect());
    }

    pub fn get_ata_ixs(
        &self,
        solvers: &[Pubkey],
        mint: &Pubkey,
    ) -> Result<Vec<Instruction>, SBError> {
        return Ok(solvers
            .iter()
            .filter(|solver| !self.does_ata_exist(solver, mint))
            .map(|solver_wo_ata| self.get_ata_instruction(solver_wo_ata, mint))
            .collect::<Vec<Instruction>>());
    }

    /// Complete bounty
    pub fn complete_bounty(
        &self,
        domain: &str,
        sub_domain: &str,
        issue_id: &u64,
        solvers: &Vec<Pubkey>,
        bounty_mint: &Pubkey,
    ) -> Result<(Bounty, String), SBError> {
        let protocol = self.get_protocol_pda();
        let relayer = self.get_relayer_pda();
        let bounty_pda = self.get_bounty_pda(domain, sub_domain, issue_id);
        let escrow_pda = self.get_escrow_pda(&bounty_pda.0);
        let fee_collector = self.get_fee_collector(bounty_mint);

        let ata_ixs = self.get_ata_ixs(solvers, bounty_mint)?;
        let atas = self.get_ata(solvers, bounty_mint)?;
        let compelete_bounty_accounts = bounty::accounts::CompleteBounty {
            payer: self.payer.pubkey(),
            protocol: protocol.0,
            fee_collector,
            relayer: relayer.0,
            bounty: bounty_pda.0,
            escrow: escrow_pda.0,
            solver1: *atas.get(0).unwrap(),
            solver2: atas.get(1).copied(),
            solver3: atas.get(2).copied(),
            solver4: atas.get(3).copied(),
            system_program: system_program::ID,
            token_program: anchor_spl::token::ID,
        };

        let complete_bounty_data = bounty::instruction::CompleteBounty {};
        let complete_bounty_ix = solana_sdk::instruction::Instruction {
            program_id: bounty::id(),
            accounts: compelete_bounty_accounts.to_account_metas(None),
            data: complete_bounty_data.data(),
        };

        let mut request = self.program.request();

        // build ata instructions
        for ix in ata_ixs {
            request = request.instruction(ix);
        }

        let (bounty, sig) = match request.instruction(complete_bounty_ix).send() {
            Ok(sig) => (
                self.get_bounty(domain, sub_domain, issue_id).unwrap(),
                sig.to_string(),
            ),
            Err(err) => {
                let protocol_data = self.get_protocol().unwrap();
                let escrow = self.get_escrow(&bounty_pda.0).unwrap();
                log::info!(
                    "Failed to complete bounty: {:?}, protocol: {}, fee collector: {}, protocol.fee_collector {}, bounty: {}, mint: {}, escrow balance {}",
                    err,
                    protocol.0.to_string(),
                    fee_collector.to_string(),
                    protocol_data.fee_collector.to_string(),
                    bounty_pda.0.to_string(),
                    bounty_mint.to_string(),
                    escrow.amount,
                );
                return Err(SBError::FailedToCompleteBounty(
                    "try complete bounty".to_string(),
                    err.to_string(),
                ));
            }
        };
        Ok((bounty, sig))
    }
}
