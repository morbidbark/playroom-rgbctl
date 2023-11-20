use stm32f4xx_hal::{
    gpio::alt::I2cCommon,
    i2c::{self, I2c, Mode},
    pac::I2C1,
    prelude::*,
    rcc::Clocks,
};

const ADDRESS: u8 = 0x28;

/// IMU various operation modes.
const CONFIG_MODE: u8 = 0b0000;
const ACCONLY_MODE: u8 = 0b0001;
const MAGONLY_MODE: u8 = 0b0010;
const GYROONLY_MODE: u8 = 0b0011;
const ACCMAG_MODE: u8 = 0b0100;
const ACCGYRO_MODE: u8 = 0b0101;
const MAGGYRO_MODE: u8 = 0b0110;
const AMG_MODE: u8 = 0b0111;
const IMU_MODE: u8 = 0b1000;
const COMPASS_MODE: u8 = 0b1001;
const M4G_MODE: u8 = 0b1010;
const NDOF_FMC_OFF_MODE: u8 = 0b1011;
const NDOF_MODE: u8 = 0b1100;

// IMU register addresses
const EUL_Heading_LSB: u8 = 0x1A; // Output data register - Yaw
const EUL_Heading_MSB: u8 = 0x1B; // Output data register - Yaw
const EUL_Roll_LSB: u8 = 0x1C; // Output data register - Roll
const EUL_Roll_MSB: u8 = 0x1D; // Output data register - Roll
const EUL_Pitch_LSB: u8 = 0x1E; // Output data register - Pitch
const EUL_Pitch_MSB: u8 = 0x1F; // Output data register - Pitch
const OPR_MODE: u8 = 0x3D; // Operation Mode

pub struct IMU {
    imu: I2c<I2C1>,
}
impl IMU {
    pub fn init(
        scl: impl Into<<I2C1 as I2cCommon>::Scl>,
        sda: impl Into<<I2C1 as I2cCommon>::Sda>,
        i2c: I2C1,
        clocks: &Clocks,
    ) -> Self {
        let imu = I2c::new(
            i2c,
            (scl, sda),
            Mode::Standard {
                frequency: 100.kHz(),
            },
            &clocks,
        );
        let mut result = Self { imu };
        result.write(OPR_MODE, NDOF_MODE).expect("Failed to set IMU config mode");
        result
    }

    pub fn orientation(&mut self) -> Result<(i16, i16, i16), i2c::Error> {
        Ok((
            self.read_i16(EUL_Pitch_LSB)?,
            self.read_i16(EUL_Heading_LSB)?,
            self.read_i16(EUL_Roll_LSB)?,
        ))
    }

    fn read_i16(&mut self, addr: u8) -> Result<i16, i2c::Error> {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;
        self.read(addr, core::slice::from_mut(&mut lsb))?;
        self.read(addr + 1, core::slice::from_mut(&mut msb))?;
        Ok((((msb as u16) << 8) | lsb as u16) as i16)
    }

    fn write(&mut self, register: u8, data: u8) -> Result<(), i2c::Error> {
        self.imu.write(ADDRESS, &[register, data])?;
        Ok(())
    }

    fn read(&mut self, register: u8, data: &mut [u8]) -> Result<(), i2c::Error> {
        self.imu.write_read(ADDRESS, &[register], data)?;
        Ok(())
    }
}