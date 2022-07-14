#[derive(Eq, Hash, PartialEq, Debug)]
#[repr(i16)]
pub enum RoleType {
    Crewmate = 0,
    Imposter = 1,
    Scientist = 2,
    Engineer = 3,
    GuardianAngel = 4,
    Shapeshifter = 5,
}
