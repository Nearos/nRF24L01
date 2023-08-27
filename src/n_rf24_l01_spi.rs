use crate::hardware_interface;


pub struct NRf24L01SpiConnection
    <
	PinIf : hardware_interface::DigitalPinInterface,
	TimerIf : hardware_interface::TimerInterface>{
    pins : PinIf,
    spi_pins : hardware_interface::SpiInterface<PinIf>,
    timer : TimerIf,
    clock_quarter_period : u64,
	
}

impl<PinIf : hardware_interface::DigitalPinInterface,
     TimerIf : hardware_interface::TimerInterface>
    NRf24L01SpiConnection<PinIf, TimerIf> {
	fn init_spi(
	    mut pins : PinIf,
	    spi_pins : hardware_interface::SpiInterface<PinIf>,
	    timer : TimerIf,
	    clock_quarter_period : u64)
	    -> Self {
	    //make sure the chip select (active low) is initially high
	    pins.write_pin(spi_pins.csn, PinIf::HIGH); 
	    Self {
		pins: pins,
		spi_pins: spi_pins,
		timer: timer,
		clock_quarter_period: clock_quarter_period,
	    }
	}

	fn txrx_raw_byte(&mut self, mut data : u8) -> u8 {
	    let mut ret : u8 = 0x00;
	    for _ in 1..8 {
		self.pins.write_pin(
		    self.spi_pins.mosi,
		    if data & 0x80 == 0 {
			PinIf::HIGH
		    }else{
			PinIf::LOW
		    });

		data <<= 1;

		self.timer.sleep_ms(self.clock_quarter_period);
		self.pins.write_pin(self.spi_pins.sck, PinIf::HIGH);

		if self.pins.read_pin(self.spi_pins.miso) == PinIf::HIGH {
		    ret |= 0x1;
		}
		ret <<= 1;
		
	
		self.timer.sleep_ms(self.clock_quarter_period);
		self.pins.write_pin(self.spi_pins.sck, PinIf::LOW);
	    }
	    ret
	}

	fn txrx_full_message(&mut self, data : [u8; 3]) -> [u8; 3] {
	    let mut ret = [0x00; 3];
	    self.pins.write_pin(self.spi_pins.csn, PinIf::LOW);
	    for i in 0..3 {
		ret[i] = self.txrx_raw_byte(data[i]);
		if i != 2 {
		    self.pins.write_pin(self.spi_pins.mosi,
					if ret[i+1] & 0x80 != 0 {
					    PinIf::HIGH
					}else{
					    PinIf::LOW
					});
		    self.timer.sleep_ms(self.clock_quarter_period);
		}
		self.timer.sleep_ms(self.clock_quarter_period);
	    }
	    self.pins.write_pin(self.spi_pins.csn, PinIf::HIGH);
	    ret
	}
}
