// This module is a hack to read additional information in the DT nodes in the
// kernel to retrieve data that is currently not exposed in the charlcd.c
// driver.

use std::fs::File;
use std::io::Result;

use byteorder::{BigEndian, ReadBytesExt};

fn read_u32_node(node_name: &str) -> Result<u32> {
    let mut f = File::open(format!(
        "/sys/devices/platform/auxdisplay/of_node/{}",
        node_name
    ))?;
    let value = f.read_u32::<BigEndian>()?;
    Ok(value)
}

pub fn display_height_chars() -> Result<u32> {
    read_u32_node("display-height-chars")
}

pub fn display_width_chars() -> Result<u32> {
    read_u32_node("display-width-chars")
}
