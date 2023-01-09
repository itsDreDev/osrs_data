#[derive(Debug, Primitive, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum MovementId {
    Standing = 808,
    Walking = 819,
    Turning180 = 820,
    Turning90 = 822,
    Running = 824,
    Jumping = 828,
    FalconStanding = 5160,
    FalconWalking = 5164,
    FalconTurning180 = 5165,
    FalconTurning90 = 5167,
    FalconRunning = 5168,
}
