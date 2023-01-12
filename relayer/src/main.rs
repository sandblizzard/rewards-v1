pub mod jobs;
pub use jobs::verification;

/// Relayer that constantly watches
/// - domains specified in the contract
/// - new PRs in the verification repo
/// - bounties in domains
fn main() {
    while true {

        // search through verification repo
    }
}
