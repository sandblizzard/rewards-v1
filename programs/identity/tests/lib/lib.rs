use solana_program_test::ProgramTest;

pub struct ProgramManager {}

impl ProgramManager {
    pub fn init() -> ProgramManager {
        return ProgramManager {};
    }

    /// add_necessary_programs adds programs to the the test context that the
    /// sure program interacts with
    pub fn add_necessary_programs(&self, ctx: &mut ProgramTest) {}
}
