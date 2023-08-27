pub trait DigitalPinInterface {
    type DigitalPin : Copy;
    type DigitalPinState : Eq;

    const HIGH : Self::DigitalPinState;
    const LOW : Self::DigitalPinState;

    fn write_pin(&mut self, pin : Self::DigitalPin, state : Self::DigitalPinState);
    fn read_pin(&self, pin : Self::DigitalPin) -> Self::DigitalPinState;

}

pub struct SpiInterface<T : DigitalPinInterface> {
    pub csn : T::DigitalPin,
    pub sck : T::DigitalPin,
    pub mosi : T::DigitalPin,
    pub miso : T::DigitalPin,
}

pub trait TimerInterface {
    fn sleep_ms(&self, ms : u64);
}
