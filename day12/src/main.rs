use std::{
    collections::{HashMap, HashSet}, fs::File, hash::Hash, io::{prelude::*, BufReader}, path::Path, usize
};

fn main() {
    println!("Hello, world!");
}

struct Garden
{
    plots: Vec<Vec<Plot>>,
    rows: usize,
    cols: usize,
}

#[derive(Eq, PartialEq, Hash)]
struct Plot
{
    row: usize,
    col: usize,
    plant: char,
}

struct Region {
    plant: char,
    perimeter: usize,
    area: usize,
}

struct Replot<'a> {
    region: usize,
    plot: &'a Plot,
}

fn _parse_file(filename: impl AsRef<Path>) -> Garden {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let rows = buf.lines().enumerate().map(|(row, line)|
        line.unwrap().chars().enumerate()
            .map(|(col, plant)|
                Plot {
                    row,
                    col,
                    plant
                })
            .collect::<Vec<Plot>>())
        .collect::<Vec<Vec<Plot>>>();

    let height = rows.len();
    let width = rows[0].len();

    Garden {
        plots: rows,
        rows: height,
        cols: width,
    }
}

fn find_free_plot<'garden>(garden: &'garden Garden, taken_plots: &HashSet<&Plot>) -> Option<&'garden Plot>
{
    let plots = garden.plots.iter()
        .flatten()
        .filter(|plot|
            !taken_plots.contains(plot))
        .collect::<Vec<&Plot>>();

    if plots.len() > 0 {
        Some(plots[0])
    } else {
        None
    }
}

fn get_neighbours<'garden>(garden: &'garden Garden, plot: &Plot) -> Vec<&'garden Plot>{

    let offsets: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    offsets.iter()
        .filter_map(|(row_offset, col_offset)|
            {
                let row = plot.row as i64 + row_offset;
                let col = plot.col as i64 + col_offset;

                if row < 0 || row >= garden.rows as i64 ||
                    col < 0 || col >= garden.cols as i64 {
                    None
                } else {
                    let neighbour = &garden.plots[row as usize][col as usize];
                    if neighbour.plant == plot.plant {
                        Some(neighbour)
                    } else {
                        None
                    }
                }
            })
        .collect::<Vec<&Plot>>()
}

fn collect_area<'garden>(garden: &'garden Garden, plot: &'garden Plot) -> HashSet<&'garden Plot> {
    let mut collected_neighbours: HashSet<&Plot> = HashSet::new();
    let mut search_neighbours: HashSet<&Plot> = HashSet::new();

    //println!("collecting plot [{}, {}] {}", plot.row, plot.col, plot.plant);

    collected_neighbours.insert(plot);
    search_neighbours.insert(plot);

    while search_neighbours.len() > 0 {

        let neighbour = search_neighbours.iter()
            .map(|x| *x)
            .take(1)
            .collect::<Vec<&Plot>>()[0];

        search_neighbours.remove(neighbour);

        let new_neighbours = get_neighbours(garden, neighbour);

        for new_neighbour in new_neighbours {

            if !collected_neighbours.contains(new_neighbour) {
                search_neighbours.insert(new_neighbour);
            }

            collected_neighbours.insert(new_neighbour);
        }
    }

    collected_neighbours
}

fn _parse1(filename: impl AsRef<Path>) -> usize {

    let garden = _parse_file(filename);

    let mut taken_plots: HashSet<&Plot> = HashSet::new();

    let mut regions: Vec<Region> = Vec::new();
    while let Some(free_plot) = find_free_plot(&garden, &taken_plots) {

        let neighbours = collect_area(&garden, free_plot);

        let perimeter = neighbours.iter()
            .map(|neighbour|
                4 - get_neighbours(&garden, *neighbour).len())
            .sum();
    
        let area = neighbours.len();

        for neighbour in neighbours {
            taken_plots.insert(neighbour);
        }

        regions.push(Region {
            perimeter,
            area,
            plant: free_plot.plant,
        });
        
    }

    let price = regions.iter()
        .map(|region|
            {
                region.area * region.perimeter
            })
        .sum();

    price
}

fn _parse2(filename: impl AsRef<Path>) -> usize {
    let garden = _parse_file(filename);

    let mut taken_plots: HashSet<&Plot> = HashSet::new();

    let mut regions: Vec<HashSet<&Plot>> = Vec::new();
    while let Some(free_plot) = find_free_plot(&garden, &taken_plots) {

        let neighbours = collect_area(&garden, free_plot);

        for neighbour in &neighbours {
            taken_plots.insert(neighbour);
        }

        regions.push(neighbours);
    }

    let mut sorted_regional_plots = regions
        .iter()
        .enumerate()
        .map(|(index, region)|
            region.iter()
                .map(|plot|
                    Replot {
                        region: index,
                        plot: *plot,
                    })
                .collect::<Vec<Replot>>())
        .flatten()
        .collect::<Vec<Replot>>();

    sorted_regional_plots.sort_unstable_by_key(|region| (region.plot.row, region.plot.col));

    let mut previous_fence_up = usize::MAX;
    let mut previous_fence_down = usize::MAX;
    let mut previous_fence_left = usize::MAX;
    let mut previous_fence_right = usize::MAX;

    let mut fences: HashMap<usize, usize> = HashMap::new();
    let mut areas: HashMap<usize, usize> = HashMap::new();

    for row in 0..(garden.cols) {
        for col in 0..(garden.cols) {
            let offset = row * garden.cols + col;
            let replot = &sorted_regional_plots[offset];

            let same_region_up = row > 0 && sorted_regional_plots[offset - garden.cols].region == replot.region;
            let same_region_down = row < (garden.rows - 1) && sorted_regional_plots[offset + garden.cols].region == replot.region;

            let fence_up = if same_region_up {
                0
            } else if previous_fence_up == replot.region {
                0
            } else {
                1
            };

            let fence_down = if same_region_down {
                0
            } else if previous_fence_down == replot.region {
                0
            } else {
                1
            };

            if let Some(region_fence) = fences.get(&replot.region) {
                fences.insert(replot.region, region_fence + fence_up + fence_down);
            } else {
                fences.insert(replot.region, fence_up + fence_down);
            }

            if let Some(region_area) = areas.get(&replot.region) {
                areas.insert(replot.region, region_area + 1);
            } else {
                areas.insert(replot.region, 1);
            }

            previous_fence_up = if same_region_up { usize::MAX } else { replot.region };
            previous_fence_down = if same_region_down { usize::MAX } else { replot.region };
        }
    }

    for col in 0..(garden.cols) {
        for row in 0..(garden.cols) {
            let offset = row * garden.cols + col;
            let replot = &sorted_regional_plots[offset];

            let same_region_left = col > 0 && sorted_regional_plots[offset - 1].region == replot.region;
            let same_region_right = col < (garden.cols - 1) && sorted_regional_plots[offset + 1].region == replot.region;

            let fence_left = if same_region_left {
                0
            } else if previous_fence_left == replot.region {
                0
            } else {
                1
            };

            let fence_right = if same_region_right {
                0
            } else  if previous_fence_right == replot.region {
                0
            } else {
                1
            };

            if let Some(region_fence) = fences.get(&replot.region) {
                fences.insert(replot.region, region_fence + fence_left + fence_right);
            } else {
                fences.insert(replot.region, fence_left + fence_right);
            }

            previous_fence_left = if same_region_left { usize::MAX } else { replot.region };
            previous_fence_right = if same_region_right { usize::MAX } else { replot.region };
        }
    }

    let result = fences.iter()
        .map(|(region, fence)|
            {
                fence * areas[region]
            })
        .sum();

    result

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result1 = _parse1("src/input");
        println!("done 1: {result1}");
        let result2 = _parse2("src/input");
        println!("done 2: {result2}");
        assert!(false);
    }
}