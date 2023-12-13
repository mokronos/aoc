use rust::read_data;

// credit to: https://topaz.github.io/paste/#XQAAAQAnCQAAAAAAAAA6nMjJFD6Qz6l42wOwWQTnPedTMuopkVEpU30SXocJrEr8RLZQ9AeZoRdOUH6TfUdwLiS3O+/2T6PmeGHkKPYMb9krBdxbe7XXI9PtzCSRqmGbRDDZ6JELOJjEIsQej/GZx3GDtxM0TVZuA8p/v8xk+wkx5sp30E3i43JvhTdDQ45FpbRYgAuSuB8giapCVJSh/BapQ4hfnmZhMTkn+w17cGa9dsZXwbAuB/x5ZnuzGewXiDYA00hrPn0NhJCNRJeLiaUx1g+gHXiOZQD7hBcfp2RJ45JUjCok7Ci+8h8GVzE+TspFMm52tELgd1/jpAhrZGnp/ruGC6ol9pR4c6334sZMUfa+VrhsM+CYTP4EBPSmQsL1JtLQmB4I/Ye8kWLL96l2hOraNmkgwPlbbGqVeT3iHVzYx5qgX/iHe/qNJ6XOhRUxTnQHtXfDljAeJOBrhInS5VmSlYi9SubIUPs+E/RsqXrFzjEtZS1sjpzU3uguA7J+KdsLemNdBGD08HrCcIF+jCF0CRcsiODyVf4LuXRiSLLwBm5f/L+zAhCUcBWulqUja71rX7CmNaLIZ1b3e/Z9kZagCVDDk4NG+xb99F4HBDjLPsRfEAqLATyhnmsya/52VEAJ9DGurnl0kuXmAJQVl4Ze/9TZ4/A6DbjXQwimqm7oKw90pvoepr0/bUcXcZw0EP8fYRBoT1gmA0zrAGFsLjcwFKElKKWySr/kFAwyPOgYNuAyS76yDecO/caAnM6pdUxKRjD8b6QqrNgNT/cwYeaj7zgkeuo6aGvSqRz0rOpxSo2mIh3Y7B1bDcMB2lFAHv5bEY9qYUs4DR+IIb121uHANWl3bo74RtONE0dL2C/Xfuj0rEHhosD8y9DxJeBxUytYZTbDT+cFATAzdNnbz/rVrMcteZt+kEX1A0JBrUWwBnEjZHjh+PuNg0VwbIIRmi/kLKKJQiW2h/9l6Yivga7+/3Z4kx19plLt2RDXsCt7ZCrXZrlXmIM3qqW7Cd+ucXiMbn7JkwHHzmd9y70wOES7azRMvYekqrF0UmY+g0ul03v+Kzk+FE0q/7M16X0=

#[allow(unused)]
const TEST_DATA: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

pub fn run12_1()
{
    let data = read_data("data/data12.txt");
    // let data = TEST_DATA;
    let total: usize = data
        .lines()
        .map(|line| Record::parse(line, 1).count_permutations())
        .sum();

    println!("Task 12_1 Solution:\nTotal: {}", total);
}

pub fn run12_2()
{
    let data = read_data("data/data12.txt");
    // let data = TEST_DATA;
    let total: usize = data
        .lines()
        .map(|line| Record::parse(line, 5).count_permutations())
        .sum();

    println!("Task 12_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Record {
    pattern: Vec<char>,
    springs: Vec<usize>,
}

impl Record {
    fn parse(line: &str, reps: usize) -> Self {
        let (pattern, springs) = line.split_once(' ').unwrap();
        let mut pattern = pattern.chars().collect::<Vec<char>>();
        pattern.push('?');
        let mut pattern = vec![pattern; reps].into_iter().flatten().collect::<Vec<char>>();
        pattern.pop();
        let springs = springs.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let springs = vec![springs; reps].into_iter().flatten().collect::<Vec<usize>>();
        Self {
            pattern,
            springs,
        }
    }

    fn count_permutations(&self) -> usize {
        let mut cache = vec![vec![None; self.springs.len() + 1]; self.pattern.len()];
        self.count(&mut cache, 0, 0)
    }

    fn count(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if i == self.pattern.len() {
            return if j == self.springs.len() { 1 } else { 0 };
        }
        if let Some(v) = cache[i][j] {
            return v;
        }
        let res = match self.pattern[i] {
            '.' => self.count(cache, i + 1, j),
            '#' => self.count_broken(cache, i, j),
            '?' => self.count(cache, i + 1, j) + self.count_broken(cache, i, j),
            _ => unreachable!(),
        };
        cache[i][j] = Some(res);
        res
    }

    fn count_broken(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if j == self.springs.len() {
            return 0;
        }
        let end_group_idx = i + self.springs[j];
        if !self.broken_group_possible(i, end_group_idx) {
            return 0;
        }
        if end_group_idx == self.pattern.len() {
            return if j == self.springs.len() - 1 { 1 } else { 0 };
        }
        self.count(cache, end_group_idx + 1, j + 1)
    }
    
    fn broken_group_possible(&self, start: usize, end: usize) -> bool {
        match end.cmp(&self.pattern.len()) {
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => self.pattern[start..end].iter().all(|&b| b!= '.'),
            std::cmp::Ordering::Less => self.pattern[start..end].iter().all(|&b| b!= '.')
                && self.pattern[end] != '#',
        }
    }
}
