#[repr(i8)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum UserState {
    Loading = 0,
    InGame = 1,
    WaitingForHost = 2
}
