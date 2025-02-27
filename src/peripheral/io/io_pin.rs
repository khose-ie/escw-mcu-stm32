#[repr(u16)]
#[derive(Clone, Copy)]
pub enum IoPin
{
    P00 = 0x0001,
    P01 = 0x0002,
    P02 = 0x0004,
    P03 = 0x0008,
    P04 = 0x0010,
    P05 = 0x0020,
    P06 = 0x0040,
    P07 = 0x0080,
    P08 = 0x0100,
    P09 = 0x0200,
    P10 = 0x0400,
    P11 = 0x0800,
    P12 = 0x1000,
    P13 = 0x2000,
    P14 = 0x4000,
    P15 = 0x8000,
}

impl IoPin
{
    pub const fn size() -> usize
    {
        16
    }
}

impl From<u16> for IoPin
{
    fn from(value: u16) -> Self
    {
        if value == Self::P00 as u16
        {
            return Self::P00;
        }
        else if value == Self::P01 as u16
        {
            return Self::P01;
        }
        else if value == Self::P02 as u16
        {
            return Self::P02;
        }
        else if value == Self::P03 as u16
        {
            return Self::P03;
        }
        else if value == Self::P04 as u16
        {
            return Self::P04;
        }
        else if value == Self::P05 as u16
        {
            return Self::P05;
        }
        else if value == Self::P06 as u16
        {
            return Self::P06;
        }
        else if value == Self::P07 as u16
        {
            return Self::P07;
        }
        else if value == Self::P08 as u16
        {
            return Self::P08;
        }
        else if value == Self::P09 as u16
        {
            return Self::P09;
        }
        else if value == Self::P10 as u16
        {
            return Self::P10;
        }
        else if value == Self::P11 as u16
        {
            return Self::P11;
        }
        else if value == Self::P12 as u16
        {
            return Self::P12;
        }
        else if value == Self::P13 as u16
        {
            return Self::P13;
        }
        else if value == Self::P14 as u16
        {
            return Self::P15;
        }
        else
        {
            Self::P15
        }
    }
}

impl Into<u16> for IoPin
{
    fn into(self) -> u16
    {
        self as u16
    }
}
