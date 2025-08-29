struct Data<'a> {
    data: [u8; 128],
    top_three: &'a [u8],
}

fn swallow(mut data: Data) {
    data.data[0] = 1;
    let data_adr = data.data.as_ptr();
    println!("moved data_adr {data_adr:p}");
    let top_three_adr = data.top_three.as_ptr();
    println!("moved top three adr {top_three_adr:p}");
    assert_eq!(0, data.top_three[0]);
}

#[test]
fn test_pin() {
    let value = [0_u8; 128];
    let data = Data {
        data: value,
        top_three: &value[0..3],
    };
    let data_adr = data.data.as_ptr();
    println!("data_adr {data_adr:p}");
    let top_three_adr = data.top_three.as_ptr();
    println!("top three adr {top_three_adr:p}");
    assert_eq!(0, data.data[0]);
    assert_eq!(0, data.top_three[0]);
    swallow(data);
}
