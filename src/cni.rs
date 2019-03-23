use glob::glob;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct IPAM {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IP4 {
    ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IPAMRes {
    ip4: IP4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CNISpec {
    #[serde(rename = "deviceID")]
    device_id: String,
    ipam: IPAM,
}

#[derive(Serialize, Deserialize, Debug)]
struct CNISpecRes {}

fn get_mac_with_vfpci(vfpci: &String) {
    let glob_physfn = format!("/sys/devices/pci*/*/{}/physfn", vfpci);
    let pfpci_g = glob(&glob_physfn).unwrap().next().unwrap();
    match pfpci_g {
        Ok(pfpci_path) => {
            let l = pfpci_path.read_link().unwrap();
            let pf = l.strip_prefix("../").unwrap();
            println!("path is {}", pf.display());
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn parse(cni_conf: String) -> CNISpec {
    serde_json::from_str(&cni_conf).unwrap()
}

pub fn ipam(c: &CNISpec) -> String {
    let exe = &c.ipam.type_;
    let res = Command::new(exe)
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());
    let res_str = String::from_utf8(res.stdout).unwrap();
    let ires: IPAMRes = serde_json::from_str(&res_str).unwrap();

    ires.ip4.ip
}

pub fn add_pair_ns(c: &CNISpec) {
    let vfpci = &c.device_id;
    get_mac_with_vfpci(&vfpci)
}
