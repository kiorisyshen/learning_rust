#[allow(dead_code)]
fn main() {
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));

        println!("home: {:?}, loopback: {:?}", home, loopback);
    }

    {
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        let coin = Coin::Quarter(UsState::Alaska);
        value_in_cents(&coin);

        let mut count = 0;
        if let Coin::Quarter(state1) = coin {
            println!("State quarter from {:?}!", state1);
        } else {
            count += 1;
            println!("count: {}", count);
        }
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }

    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {
        }
    }
}
