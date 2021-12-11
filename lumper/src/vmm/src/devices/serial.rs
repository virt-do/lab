use std::io::{stdout, Error, Result, Stdout};
use std::ops::Deref;

use vm_superio::serial::NoEvents;
use vm_superio::{Serial, Trigger};
use vmm_sys_util::eventfd::EventFd;

pub struct EventFdTrigger(EventFd);
impl Trigger for EventFdTrigger {
    type E = Error;

    fn trigger(&self) -> Result<()> {
        self.write(1)
    }
}
impl Deref for EventFdTrigger {
    type Target = EventFd;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl EventFdTrigger {
    pub fn new(flag: i32) -> Result<Self> {
        Ok(EventFdTrigger(EventFd::new(flag)?))
    }
    pub fn try_clone(&self) -> Result<Self> {
        Ok(EventFdTrigger((**self).try_clone()?))
    }
}

pub(crate) struct LumperSerial {
    eventfd: EventFdTrigger,
    pub serial: Serial<EventFdTrigger, NoEvents, Stdout>,
}

impl LumperSerial {
    pub fn new() -> Result<Self> {
        let eventfd = EventFdTrigger::new(libc::EFD_NONBLOCK).unwrap();

        Ok(LumperSerial {
            eventfd: eventfd.try_clone()?,
            serial: Serial::new(eventfd.try_clone()?, stdout()),
        })
    }

    pub fn eventfd(&self) -> Result<EventFd> {
        Ok(self.eventfd.try_clone()?.0)
    }

    pub fn serial(&self) -> &Serial<EventFdTrigger, NoEvents, Stdout> {
        &self.serial
    }
}
