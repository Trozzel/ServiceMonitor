use svcmon::database::crudops::get_json_statuses;
use svcmon::hosts::Hosts;

pub fn main() {
    let hosts = Hosts::init_from_file().unwrap().get_hosts();
    let mut i = 0;
    for host in hosts {
        for _ in host.services {
            println!("Service #: {}", i);
            i += 1;
        }
    }

    let statuses = get_json_statuses();

    for status in statuses.unwrap() {
        println!("hostname: {}, service: {}, status: {}", 
                 status.hostname, status.name, status.active_status.unwrap());
    }
}
