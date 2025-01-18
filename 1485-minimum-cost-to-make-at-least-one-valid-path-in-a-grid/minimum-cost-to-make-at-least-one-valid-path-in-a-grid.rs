use std::collections::VecDeque;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // Convert row and column to a single index
        fn cell_index(r: usize, c: usize, n: usize) -> usize {
            r * n + c
        }

        // Check if a cell is within bounds
        fn is_valid_cell(r: i32, c: i32, m: usize, n: usize) -> bool {
            r >= 0 && r < m as i32 && c >= 0 && c < n as i32
        }

        // Directions: right, left, down, up
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        // Distance array to track the minimum cost to reach each cell
        let mut dist = vec![i32::MAX; m * n];
        dist[cell_index(0, 0, n)] = 0;

        // Two queues for BFS with prioritization
        let mut current_queue: VecDeque<usize> = VecDeque::new();
        let mut next_queue: VecDeque<usize> = VecDeque::new();

        current_queue.push_back(cell_index(0, 0, n));
        let mut current_cost = 0;

        while !current_queue.is_empty() {
            while let Some(cell) = current_queue.pop_front() {
                // Early check: Skip if the cell has already been processed with a smaller cost
                if dist[cell] > current_cost {
                    continue;
                }

                let r = cell / n;
                let c = cell % n;

                for (dir, (dx, dy)) in directions.iter().enumerate() {
                    let nr = r as i32 + dx;
                    let nc = c as i32 + dy;

                    if is_valid_cell(nr, nc, m, n) {
                        let next = cell_index(nr as usize, nc as usize, n);
                        let mut cost = 1;

                        // If the current cell's direction matches, no cost
                        if grid[r][c] == (dir as i32 + 1) {
                            cost = 0;
                        }

                        if dist[next] > current_cost + cost {
                            dist[next] = current_cost + cost;
                            if cost == 0 {
                                current_queue.push_back(next);
                            } else {
                                next_queue.push_back(next);
                            }
                        }
                    }
                }
            }

            // Swap the queues and increment cost
            std::mem::swap(&mut current_queue, &mut next_queue);
            current_cost += 1;
        }

        // Return the minimum cost to reach the bottom-right corner
        dist[cell_index(m - 1, n - 1, n)]
    }
}