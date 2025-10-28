#[derive(Clone, Copy, Debug)]
pub enum ZoomDirection
{
    Inwards,
    Outwards
}

impl ZoomDirection
{
    pub fn forward(self) -> bool
    {
        match self
        {
            Self::Inwards => true,
            Self::Outwards => false
        }
    }
}