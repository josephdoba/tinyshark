pub fn capture() {
    let device = pcap::Device::lookup()
        .expect("device lookup failed")
        .expect("no device available");
    println!("Using device GUID: {}", device.name);
    println!("Description: {:?}", device.desc);
    println!("addresses: {:?}", device.addresses);
    println!("flags: {:?}", device.flags);

    // setup the capture:
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    // retrieve packet and print out bytes:
    println!("{:?}", cap.next_packet());

    // show packet header
    let mut count = 0;
    cap.for_each(None, |packet| {
        println!("Got {:?}", packet.header);
        count += 1;
        if count > 100 {
            panic!("ow");
        }
    })
    .unwrap();
}
