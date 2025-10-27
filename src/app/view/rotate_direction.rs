#[derive(Clone, Copy, Debug)]
pub enum RotateDirection
{
    Left,
    Right
}

impl RotateDirection
{
    pub fn forward(self) -> bool
    {
        match self
        {
            Self::Right => true,
            Self::Left => false
        }
    }
}