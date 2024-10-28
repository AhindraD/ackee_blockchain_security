fn _main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
        Unknown,
    }

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
    let _unknown = IpAddr::Unknown;
}
