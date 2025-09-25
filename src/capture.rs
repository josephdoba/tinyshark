pub fn capture() {
    let device = pcap::Device::lookup()
        .expect("device lookup failed")
        .expect("no device available");
    println!("Using device GUID: {}", device.name);

    // setup the capture:
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    // retrieve packet and print out bytes:
    println!("{:?}", cap.next_packet());
}
