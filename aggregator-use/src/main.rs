use aggregator::aggregate;
#[derive(Debug)]
struct Car {
    cylinders: u8,
}

#[derive(Debug)]
struct Bike {
    gears: u8,
}

#[derive(Debug)]
#[aggregate]
enum Vehicle {
    Bike(Bike),
    Car(Car),
}

// The following impl blocks should be written by the macro
// impl From<Bike> for Vehicle {
//     fn from(b: Bike) -> Self {
//         Self::Bike(b)
//     }
// }

// impl From<Car> for Vehicle {
//     fn from(c: Car) -> Self {
//         Self::Car(c)
//     }
// }

fn main() {
    let balance_bike = Bike { gears: 0 };
    let v_bike: Vehicle = balance_bike.into();

    let rusty_old_car = Car { cylinders: 4 };
    let v_car: Vehicle = rusty_old_car.into();

    println!("The balance_bike: {:?}", v_bike);
    println!("The rusty old car: {:?}", v_car);
}
