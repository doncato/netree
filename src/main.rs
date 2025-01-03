use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use network_interface::V4IfAddr;
use network_interface::V6IfAddr;
use network_interface::Addr;

fn load_interfaces() {
    let interfaces = NetworkInterface::show().unwrap(); // FIXME

    for itf in interfaces.iter() {
        let name: &String = &itf.name;
        let mut ips: String = String::new();
        for itf_addr in &itf.addr {
            match itf_addr {
                Addr::V4(V4IfAddr {
                    ip,
                    broadcast: _,
                    netmask
                }) => {
                    ips.push_str(&ip.to_string());
                    if let Some(netmask) = netmask {
                        let bits: u32 = netmask.to_bits();
                        let count: u32 = u32::count_ones(bits);
                        ips.push_str("/");
                        ips.push_str(&count.to_string());
                    }

                    ips.push_str(" ");
                },
                Addr::V6(V6IfAddr {
                    ip,
                    broadcast: _,
                    netmask
                }) => {
                    ips.push_str(&ip.to_string());
                    if let Some(netmask) = netmask {
                        let bits: u128 = netmask.to_bits();
                        let count: u32 = u128::count_ones(bits);
                        ips.push_str("/");
                        ips.push_str(&count.to_string());
                    }

                    ips.push_str(" ");
                },
            }
        }
        
        println!("├── {}: {}", name, ips);
    }
    
}

fn main() {
    println!("○");

    load_interfaces();
}
