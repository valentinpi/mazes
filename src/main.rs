type Maze = Vec<Vec<bool>>;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

fn main() {
    let n = 10000;
    // All unvisited
    let mut maze: Maze = vec![vec![false; 2*n+1]; 2*n+1];
    maze[1][0] = true;
    maze[1][1] = true;
    //maze[2*n-1][2*n-1] = true;
    maze[2*n-1][2*n] = true;

    make_maze(&mut maze, n as i64);
    //print_maze(&maze);
}

// Aldous-Broder algorithm
// Assume maze is quadratic
fn make_maze(maze: &mut Maze, n: i64) {
    let mut p = Point { x: 1, y: 1 };
    let mut unvisited: Vec<Point> = vec![Point { x: 1, y: 1 }];

    // Helper Closure
    let inside_maze = |x, y| x >= 0 && x <= 2*n && y >= 0 && y <= 2*n;
    while !unvisited.is_empty() {
        if maze[p.x as usize][p.y as usize] {
            p = unvisited.pop().unwrap();
        }
        maze[p.x as usize][p.y as usize] = true;

        // Calculate accessable neighbors
        let mut neighbors: Vec<Point> = vec![];
        if inside_maze(p.x,   p.y-2) && !maze[(p.x  ) as usize][(p.y-2) as usize] { neighbors.push(Point { x: p.x, y: p.y-2 }); }
        if inside_maze(p.x+2, p.y  ) && !maze[(p.x+2) as usize][(p.y  ) as usize] { neighbors.push(Point { x: p.x+2, y: p.y }); }
        if inside_maze(p.x,   p.y+2) && !maze[(p.x  ) as usize][(p.y+2) as usize] { neighbors.push(Point { x: p.x, y: p.y+2 }); }
        if inside_maze(p.x-2, p.y  ) && !maze[(p.x-2) as usize][(p.y  ) as usize] { neighbors.push(Point { x: p.x-2, y: p.y }); }
        if neighbors.is_empty() {
            continue;
        }

        let next = rand::random::<u64>() % (neighbors.len() as u64);
        let q = neighbors[next as usize];
        neighbors.remove(next as usize);
        for neighbor in neighbors {
            unvisited.push(neighbor);
        }

        // Remove wall
             if q.x == p.x   && q.y == p.y-2 { maze[(p.x  ) as usize][(p.y-1) as usize] = true; }
        else if q.x == p.x+2 && q.y == p.y   { maze[(p.x+1) as usize][(p.y  ) as usize] = true; }
        else if q.x == p.x   && q.y == p.y+2 { maze[(p.x  ) as usize][(p.y+1) as usize] = true; }
        else if q.x == p.x-2 && q.y == p.y   { maze[(p.x-1) as usize][(p.y  ) as usize] = true; }
        p = q;
    }
}

fn print_maze(maze: &Maze) {
    for col in 0..maze.len() {
        for row in 0..maze.len() {
            if maze[row][col] {
                print!("0")
            }
            else {
                print!("1")
            }
        }
        println!("")
    }
}
