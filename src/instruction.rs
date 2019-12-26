use num_derive::ToPrimitive;

/// ST7735 instructions.
#[derive(ToPrimitive)]
pub enum Instruction {
    /// No Operation
    NOP = 0x00,
    /// Software Reset
    SWRESET = 0x01,
    /// Read Display ID
    RDDID = 0x04,
    /// Read Display Status
    RDDST = 0x09,
    /// Read Display Power
    RDDPM = 0x0A,
    /// Sleep in & booster off
    SLPIN = 0x10,
    /// Sleep out & booster on
    SLPOUT = 0x11,
    /// Partial mode on
    PTLON = 0x12,
    /// Partial off (Normal)
    NORON = 0x13,
    /// Display inversion off
    INVOFF = 0x20,
    /// Display inversion on
    INVON = 0x21,
    /// Display off
    DISPOFF = 0x28,
    /// Display on
    DISPON = 0x29,
    /// Column address set
    CASET = 0x2A,
    /// Row address set
    RASET = 0x2B,
    /// Memory write
    RAMWR = 0x2C,
    /// Memory read
    RAMRD = 0x2E,
    /// Partial start/end address set
    PTLAR = 0x30,
    /// Interface pixel format
    COLMOD = 0x3A,
    /// Memory data access control
    MADCTL = 0x36,
    /// In normal mode (Full colors)
    FRMCTR1 = 0xB1,
    /// In Idle mode
    FRMCTR2 = 0xB2,
    /// In partial mode + Full colors
    FRMCTR3 = 0xB3,
    /// Display inversion control
    INVCTR = 0xB4,
    /// Display function setting
    DISSET5 = 0xB6,
    /// Power control setting
    PWCTR1 = 0xC0,
    /// Power control setting
    PWCTR2 = 0xC1,
    /// Power control setting - In normal mode (Full colors)
    PWCTR3 = 0xC2,
    /// Power control setting - In Idle mode (8-colors)
    PWCTR4 = 0xC3,
    /// Power control setting - In partial mode + Full colors
    PWCTR5 = 0xC4,
    /// VCOM control 1
    VMCTR1 = 0xC5,
    /// Read ID1
    RDID1 = 0xDA,
    /// Read ID2
    RDID2 = 0xDB,
    /// Read ID3
    RDID3 = 0xDC,
    /// ??
    RDID4 = 0xDD,
    /// Power control setting - In partial mode + Idle
    PWCTR6 = 0xFC,
    /// Gamma adjustment (+ polarity)
    GMCTRP1 = 0xE0,
    /// Gamma adjustment (- polarity)
    GMCTRN1 = 0xE1,
}
