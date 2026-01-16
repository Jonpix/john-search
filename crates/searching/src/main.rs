use searching::search_algs::{a_star_manhattan, dijkstra};
use searching::{bfs, Grid};

fn main() {
    for i in 800..1000 {
        let seed = i as u64;
        let grid = Grid::from_seed(seed, 50, 10, 30, 20, false);
        let start = grid.get_start();
        let finish = grid.get_finish();
        let bfs_result = bfs(&grid, start, finish);

        let dijkstra_result = dijkstra(&grid, start, finish);
        if let Some(dijkstra_result) = dijkstra_result {
            println!("seed: {}, dijkstra:\n{}", seed, grid.render(&dijkstra_result.path(), &dijkstra_result.expanded_order()));
        }

        let manhattan_result = a_star_manhattan(&grid, start, finish);
        if let Some(manhattan_result) = manhattan_result {
            println!("seed: {}, manhattan:\n{}", seed, grid.render(&manhattan_result.path(), &manhattan_result.expanded_order()));
        }
    }
}
