use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "Time:      7  15   30
Distance:  9  40  200
";

// d = v * travel
// v = startup
// total_time = startup + travel
// plot d vs startup
// d = startup * (total_time - startup) = total_time * startup - startup^2

pub fn run6_1()
{
    let data = read_data("data/data6.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let races = parse_data(&data);

    for race in &races {
        println!("Time: {}, Distance {}", race.time, race.distance);
    }
    
    let options = get_options(&races);
    total = options.iter().product::<usize>();

    println!("Task 6_1 Solution:\nTotal: {}", total);
}

pub fn run6_2()
{
    let data = read_data("data/data6.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let race = parse_data_single(&data);

    println!("Time: {}, Distance {}", race.time, race.distance);
    
    let races = vec![race];
    let options = get_options(&races);
    total = options.iter().product::<usize>();

    println!("Task 6_2 Solution:\nTotal: {}", total);
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn check(&self) -> usize {
        let mut counter = 0;
        for startup in 0..self.time {
            let dist = self.time * startup - startup * startup;
            if dist > self.distance {
                counter += 1;
            }
        }
        counter
    }
}

fn parse_data (data: &str) -> Vec<Race> {

    let mut races = Vec::new();

    let times = data.lines().nth(0).unwrap().split("Time:").nth(1).unwrap()
        .split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let distances = data.lines().nth(1).unwrap().split("Distance:").nth(1).unwrap()
        .split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    for (time, distance) in times.iter().zip(distances.iter()) {
        races.push(
            Race {
                time: *time,
                distance: *distance,
            });
    }

    races
}

fn parse_data_single (data: &str) -> Race {

    let time = data.lines().nth(0).unwrap().split("Time:").nth(1).unwrap()
        .replace(" ", "").parse::<u64>().unwrap();
    let distance = data.lines().nth(1).unwrap().split("Distance:").nth(1).unwrap()
        .replace(" ", "").parse::<u64>().unwrap();

    Race {
        time,
        distance,
    }
}

fn get_options (races: &Vec<Race>) -> Vec<usize> {
    let mut options = Vec::new();
    races.iter().for_each(|x| options.push(x.check()));
    options
}
