use anchor_client::anchor_lang::accounts::sysvar;
use anchor_client::anchor_lang::InstructionData;
use anchor_client::anchor_lang::ToAccountMetas;
use anchor_client::solana_sdk::pubkey;
use rand::prelude::*;

use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, pubkey::*,
        signature::Keypair, signer::Signer,
    },
    *,
};
use anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount, *};

use bounty::{self, state::Bounty};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_associated_token_account::solana_program::rent;
use spl_associated_token_account::solana_program::sysvar::SysvarId;
/// Bounty is the SDK for the bounty program
use std::{rc::Rc, result::Result, str::FromStr, sync::Arc};

use crate::accounts::get_bounty_denomination_pda;
use crate::accounts::get_bounty_pda;
use crate::accounts::get_domain_pda;
use crate::accounts::get_escrow_pda;
use crate::accounts::get_fee_collector_pda;
use crate::accounts::get_protocol_pda;
use crate::accounts::get_relayer_pda;
use crate::accounts::get_sand_token_pda;
use crate::utils::{get_bounty_connection, get_key_from_env, load_keypair, SBError};

pub struct BountySdk {
    pub program: Program<Arc<Keypair>>,
    pub cluster: Cluster,
    pub payer: Keypair,
}

pub fn get_bounty(issue_id: &str) -> Result<bounty::state::Bounty, SBError> {
    let (program, cluster) = get_bounty_connection()?;
    let bounty_pda = get_bounty_pda(issue_id);

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
    pub fn new(cluster: Option<Cluster>) -> Result<Arc<BountySdk>, SBError> {
        let payer = load_keypair().unwrap();
        let payer_rc = Arc::new(payer);
        let cluster = match cluster {
            Some(cluster) => cluster,
            None => {
                let cluster = get_key_from_env("CLUSTER")?;
                match Cluster::from_str(&cluster.to_string()) {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(SBError::CouldNotGetEnvKey(
                            "get_program_client".to_string(),
                            "CLUSTER".to_string(),
                            err.to_string(),
                        ))
                    }
                }
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

    /// get_sand_token_mint
    fn get_sand_token_mint(&self) -> Result<Pubkey, SBError> {
        let sand_token_mint = get_key_from_env("SAND_TOKEN_MINT")?;
        Ok(Pubkey::from_str(&sand_token_mint).unwrap())
    }

    /// initialize_contract
    ///
    /// initialize the contract with the sand token mint and the nft collection
    pub fn initialize_contract(&self, sand_token_mint: &Pubkey, nft_collection: &Pubkey) {
        let protocol_pda = get_protocol_pda();
        let sand_token_account_pda = get_sand_token_pda(sand_token_mint);
        let accounts = bounty::accounts::Initialize {
            protocol_owner: self.payer.pubkey(),
            protocol: protocol_pda.0,
            sand_mint: *sand_token_mint,
            system_program: system_program::ID,
            token_program: token::ID,
        };

        let data = bounty::instruction::Initialize {};

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts,
            data: data.into(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully initialized contract {}. {}",
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    pub fn add_bounty_denomination(&self, mint: &Pubkey) {
        let sand_token_mint = self.get_sand_token_mint().unwrap();
        let protocol_pda = get_protocol_pda();
        let denomination_pda = get_bounty_denomination_pda(mint);
        let accounts = bounty::accounts::AddBountyDenomination {
            creator: self.payer.pubkey(),
            protocol: protocol_pda.0,
            mint: *mint,
            denomination: denomination_pda.0,
            fee_collector: get_fee_collector_pda(mint).0,
            token_program: token::ID,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::AddBountyDenomination {};

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully added bounty denomination contract {}. {}",
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// deactivate bounty denomination
    ///
    ///
    pub fn deactivate_bounty_denomination(&self, mint: &Pubkey) {
        let denomination_pda = get_bounty_denomination_pda(mint);
        let accounts = bounty::accounts::DeactivateBountyDenomination {
            creator: self.payer.pubkey(),
            mint: *mint,
            denomination: denomination_pda.0,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::DeactivateBountyDenomination {};

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully deactivated bounty denomination contract {}. {}",
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// add_relayer
    ///
    ///
    pub fn add_relayer(&self, relayer_address: &Pubkey) {
        let protocol_pda = get_protocol_pda();
        let relayer_pda = get_relayer_pda(&relayer_address);
        let accounts = bounty::accounts::AddRelayer {
            signer: self.payer.pubkey(),
            protocol: protocol_pda.0,
            relayer: relayer_pda.0,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::AddRelayer {
            relayer_address: *relayer_address,
        };

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully added relayer {} to program {}. {}",
                relayer_address.to_string(),
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// deactivate_relayer
    pub fn deactivate_relayer(&self, relayer_address: &Pubkey) {
        let protocol_pda = get_protocol_pda();
        let relayer_pda = get_relayer_pda(&relayer_address);
        let accounts = bounty::accounts::RemoveRelayer {
            signer: self.payer.pubkey(),
            protocol: protocol_pda.0,
            relayer: relayer_pda.0,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::RemoveRelayer {};

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully deactivated relayer {} to program {}. {}",
                relayer_address.to_string(),
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// create domain
    ///
    /// # Arguments
    /// * platform:
    /// * organization:
    /// * team:
    /// * domain_type:
    ///
    pub fn create_domain(
        &self,
        platform: &String,
        organization: &String,
        team: &String,
        domain_type: &String,
    ) {
        let protocol_pda = get_protocol_pda();
        let domain_pda = get_domain_pda(platform, organization, team, domain_type);
        let accounts = bounty::accounts::CreateDomain {
            creator: self.payer.pubkey(),
            protocol: protocol_pda.0,
            domain: domain_pda.0,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::CreateDomain {
            platform: platform.clone(),
            organization: organization.clone(),
            team: team.clone(),
            domain_type: domain_type.clone(),
        };

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully created a bounty domain {} to program {}. {}",
                domain_pda.0.to_string(),
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// deactivate domain
    pub fn deactivate_domain(
        &self,
        platform: &String,
        organization: &String,
        team: &String,
        domain_type: &String,
    ) {
        let domain_pda = get_domain_pda(platform, organization, team, domain_type);
        let accounts = bounty::accounts::DeactivateDomain {
            signer: self.payer.pubkey(),
            domain: domain_pda.0,
            system_program: system_program::ID,
        };

        let data = bounty::instruction::DeactivateDomain {};

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully deactivated bounty domain {} on program {}. {}",
                domain_pda.0.to_string(),
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// deactivate domain
    pub fn create_bounty(&self, domain: &Pubkey, mint: &Pubkey, bounty_amount: u64) {
        let bounty_id = nano_id::base64::<32>();
        let protocol_pda = get_protocol_pda();
        let bounty_pda = get_bounty_pda(&bounty_id);
        let bounty_denomination_pda = get_bounty_denomination_pda(&mint);
        let escrow_pda = get_escrow_pda(&bounty_pda.0);
        let accounts = bounty::accounts::CreateBounty {
            creator: self.payer.pubkey(),
            protocol: protocol_pda.0,
            mint: *mint,
            bounty: bounty_pda.0,
            domain: *domain,
            creator_account: self.payer.pubkey(),
            bounty_denomination: bounty_denomination_pda.0,
            escrow: escrow_pda.0,
            system_program: system_program::ID,
            token_program: token::ID,
            rent: rent::Rent::id(),
        };

        let data = bounty::instruction::CreateBounty {
            id: bounty_id,
            bounty_amount,
        };

        let ix = Instruction {
            program_id: bounty::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        };

        match self.program.request().instruction(ix).send() {
            Ok(res) => log::info!(
                "Successfully deactivated bounty domain {} on program {}. {}",
                bounty_pda.0.to_string(),
                bounty::id().to_string(),
                res
            ),
            Err(err) => log::error!("Failure. cause: {}", err.to_string()),
        };
    }

    /// complete_bounty
    ///
    /// try to complete a bounty
    pub fn complete_bounty(
        &self,
        relayer_address: &Pubkey,
        issue_id: &str,
        solvers: &Vec<Pubkey>,
        bounty_mint: &Pubkey,
    ) -> Result<(Bounty, String), SBError> {
        // get pdas
        let protocol = get_protocol_pda();
        let cluster = Cluster::Devnet;
        let relayer = get_relayer_pda(relayer_address);
        let bounty_pda = get_bounty_pda(issue_id);
        let escrow_pda = get_escrow_pda(&bounty_pda.0);
        let fee_collector = get_fee_collector_pda(bounty_mint);
        let denomination_pda = get_bounty_denomination_pda(bounty_mint);

        let ata_ixs = self.get_ata_ixs(solvers, bounty_mint)?;
        let atas = self.get_ata(solvers, bounty_mint)?;
        let compelete_bounty_accounts = bounty::accounts::CompleteBounty {
            payer: self.payer.pubkey(),
            protocol: protocol.0,
            fee_collector: fee_collector.0,
            bounty_denomination: denomination_pda.0,
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

        let mut request = RequestBuilder::from(bounty::id(), cluster.url(), &self.payer, None);

        // build ata instructions
        for ix in ata_ixs {
            request = request.instruction(ix);
        }
        let (bounty, sig) = match request.instruction(complete_bounty_ix).send() {
            Ok(sig) => (self.get_bounty(issue_id).unwrap(), sig.to_string()),
            Err(err) => {
                let escrow = self.get_escrow(&bounty_pda.0).unwrap();
                log::info!(
                    "Failed to complete bounty: {:?}, protocol: {}, fee collector: {}, bounty: {}, mint: {}, escrow balance {}",
                    err,
                    protocol.0.to_string(),
                    fee_collector.0.to_string(),
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

    pub fn get_bounty(&self, issue_id: &str) -> Result<bounty::state::Bounty, SBError> {
        log::debug!("[bounty_sdk] get_bounty for issue_id={}", &issue_id);

        let bounty_pda = anchor_client::solana_sdk::pubkey::Pubkey::find_program_address(
            &[
                bounty::utils::BOUNTY_SEED.as_bytes(),
                issue_id.as_bytes().as_ref(),
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
}
