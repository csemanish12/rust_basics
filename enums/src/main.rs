fn main() {
    #[derive(Debug)]
    enum IpAddressKind{
        V4,
        V6
    }
    // can only take one variant at a time

    let _four = IpAddressKind::V4;
    let _six = IpAddressKind::V6;

    fn route(ip_kind: IpAddressKind){
        println!("ip_kind: {:?}", ip_kind);
    }

    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    // using struct
    struct  IpAddress{
        kind: IpAddressKind,
        address: String,
    }

    let home = IpAddress{
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress{
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    }; 

    // using enum
    enum IpAddressKindV2{
        V4(String),
        V6(String)
    }
    let home_v2 = IpAddressKindV2::V4(String::from("127.0.0.1"));
    let loopback_v2 = IpAddressKindV2::V6(String::from("::.1"));


    // enhanced enum
    #[derive(Debug)]
    enum IpAddressKindV3{
        V4(u8,u8,u8,u8),
        V6(String)
    }
    let home_v3 = IpAddressKindV3::V4(127,0,0,1);
    let loopback_v2 = IpAddressKindV2::V6(String::from("::.1"));
    println!("home v3: {:?}", home_v3);
}
