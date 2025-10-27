#[derive(Clone, Copy, Debug)]
pub enum MoveDirection
{
    Up,
    Down,
    Left,
    Right
}

impl MoveDirection
{
    pub fn axis(self) -> bool
    {
        match self
        {
            Self::Up | Self::Down => true,
            Self::Left | Self::Right => false
        }
    }

    pub fn forward(self) -> bool
    {
        match self
        {
            Self::Right | Self::Up => true,
            Self::Left | Self::Down => false
        }
    }
}