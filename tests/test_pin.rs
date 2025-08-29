use std::pin::{Pin, pin};

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

fn swallow_pin(mut data: Pin<&mut Data>) {
    data.data[0] = 1;
    let data_adr = data.data.as_ptr();
    println!("not moved data_adr {data_adr:p}");
    let top_three_adr = data.top_three.as_ptr();
    println!("not moved top three adr {top_three_adr:p}");
    assert_eq!(1, data.data[0]);
    assert_eq!(0, data.top_three[0]);
}

#[test]
fn test_not_pin() {
    let value = [0_u8; 128];
    let value_adr = value.as_ptr();
    let value_slice_adr = value[0..3].as_ptr();
    assert_eq!(value_adr, value_slice_adr);
    let data = Data {
        data: value,
        top_three: &value[0..3],
    };
    assert_eq!(value_slice_adr, data.top_three.as_ptr());
    assert_ne!(data.data.as_ptr(), data.top_three.as_ptr());

    let data_adr = data.data.as_ptr();
    println!("data_adr {data_adr:p}");
    let top_three_adr = data.top_three.as_ptr();
    println!("top three adr {top_three_adr:p}");
    assert_eq!(0, data.data[0]);
    assert_eq!(0, data.top_three[0]);
    swallow(data);
}

#[test]
fn test_pin_value() {
    let value = [0_u8; 128];
    let data = pin!(Data {
        data: value,
        top_three: &value[0..3],
    });
    let data_adr = data.data.as_ptr();
    println!("pin data_adr {data_adr:p}");
    let top_three_adr = data.top_three.as_ptr();
    println!("pin top three adr {top_three_adr:p}");
    assert_eq!(0, data.data[0]);
    assert_eq!(0, data.top_three[0]);
    swallow_pin(data);
}
