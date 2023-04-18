use rand::Rng;

fn main() {
    let runs = 10_000_000;
    let spots = 24;
    let mut rng = rand::thread_rng();

    let eggs_chances = [
        0.1083,
        0.1083,
        0.0902,
        0.0902,
        0.0722,
        0.0722,
        0.0722,
        0.0541,
        0.0541,
        0.0541,
        0.0361,
        0.027 ,
        0.027 ,
        0.027 ,
        0.027 ,
        0.027 ,
        0.018 ,
        0.0144,
        0.0108,
        0.009 ,
    ];

    let eggs_names = [
        "Purple Egg",
        "Rusted Egg",
        "Purple Teethy Egg",
        "Pink Egg",
        "Plasma Spot Egg",
        "Lava Egg",
        "Cracked Moon Egg",
        "Moon's Egg",
        "Egg of the Moon",
        "Jupiter's Egg",
        "Egg of the Sun",
        "Pink Shelled Egg",
        "Emerald Egg",
        "Mummy Egg",
        "Blue Sodalite Egg",
        "Winter Blue Gem Egg",
        "Noob Egg",
        "UFO Egg",
        "Gem Egg",
        "Despacito Spider Egg",
    ];

    let mut egg_count = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    println!("This will take an estimated {} seconds", 0.000063576498 * runs as f64);// Based on my specs so might be really out for some really old/new stuff

    let start = std::time::Instant::now();

    // Simulate egg spawn
    for _ in 1..runs {
        let mut spawned = 0;

        for (i, decimal_percentage) in eggs_chances.iter().enumerate() {
            for _ in 1..spots-spawned {
                if rng.gen::<f64>() <= *decimal_percentage {
                    spawned += 1;
                    egg_count[i] += 1;
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Took: {:?}", duration);

    // output
    for (i, num) in egg_count.iter().enumerate() {
        println!("{}: {}%, ({})",
            eggs_names[i],
            (*num as f64 / runs as f64) * 100.0,
            num
        )
    }
}
