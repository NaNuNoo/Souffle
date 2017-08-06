mod arp;

pub use arp::send_arp;
pub use arp::recv_arp;

fn main() {
  recv_arp();
}
