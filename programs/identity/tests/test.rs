pub mod lib;

pub use lib::ProgramManager;
use solana_program_test::*;

#[tokio::test]
async fn test_manage_relayers() {
    let _program_manager = ProgramManager::init();
    println!("Hello there")
}
