use anyhow::Result;
use dlpi::DlpiHandle;

pub fn open(name: &str) -> Result<DlpiHandle> {
    let p = dlpi::open(name, dlpi::sys::DLPI_RAW)?;
    dlpi::bind(p, 0x86dd)?; //XXX ?
    dlpi::promisc_on(p, dlpi::sys::DL_PROMISC_MULTI)?;
    dlpi::promisc_on(p, dlpi::sys::DL_PROMISC_SAP)?;
    dlpi::promisc_on(p, dlpi::sys::DL_PROMISC_PHYS)?;
    Ok(p)
}
