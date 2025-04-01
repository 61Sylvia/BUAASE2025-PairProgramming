use wasm_bindgen::prelude::*;
use std::collections::{HashSet, VecDeque};

const N: usize = 8;
const DIRECTIONS: [(i32, i32, i32); 4] = [(0, 1, 0), (0, -1, 2), (-1, 0, 1), (1, 0, 3)];

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: Vec<i32>, fruit: Vec<i32>, barriers: Vec<i32>) -> i32 {

    let mut maze = vec![0; (N + 1) * (N + 1)];

    for i in (0..barriers.len()).step_by(2) {
        let x = barriers[i] as usize;
        let y = barriers[i + 1] as usize;
        maze[x * (N + 1) + y] = 1; 
    }

    let start = (snake[0] as usize, snake[1] as usize);
    let end = (fruit[0] as usize, fruit[1] as usize);

    if let Some(direction) = bfs_find_path(&maze, &snake, start, end) {
        return direction;
    }

    -1 
}

fn bfs_find_path(maze: &[i32], snake: &[i32], start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_snake_body = snake.to_vec();
    queue.push_back((start.0, start.1, None, initial_snake_body));

    while let Some((x, y, first_move, snake_body)) = queue.pop_front() {

        if (x, y) == end {
            return first_move; 
        }

        //蛇原先第二节身体的x,y，需保证其不和现在的头碰撞
        let second_body_x = snake_body[2];
        let second_body_y = snake_body[3];
        //println!("snake_body_len: {}", snake_body.len());
        //println!("second_body: ({}, {})", second_body_x, second_body_y);

        for &(dx, dy, dir) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            //未超界
            if nx >= 1 && nx <= N as i32 && ny >= 1 && ny <= N as i32 {
                let ni = nx as usize * (N + 1) + ny as usize;

                //不是障碍物且未访问过且头身未碰撞
                if maze[ni] == 0
                    && !visited.contains(&(nx as usize, ny as usize))
                    && !(nx == second_body_x && ny == second_body_y) 
                {
                    visited.insert((nx as usize, ny as usize));

                    let mut new_snake_body = vec![nx as i32, ny as i32];
                    new_snake_body.extend_from_slice(&snake_body);
                    new_snake_body.pop();
                    new_snake_body.pop();

                    queue.push_back((nx as usize, ny as usize, first_move.or(Some(dir)), new_snake_body));
                }
            }
        }
    }

    None 
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greedy_snake_barriers_checker(
        initial_snake: Vec<i32>,
        mut food_num: i32,
        foods: Vec<i32>,
        barriers: Vec<i32>,
        access: i32, //是否可达
    ) -> i32 {
        let mut current_snake = initial_snake.clone();
        let mut current_foods = foods.clone();
        let mut barriers_list = Vec::new();

        for i in (0..barriers.len()).step_by(2) {
            let x = barriers[i];
            let y = barriers[i + 1];
            barriers_list.push((x, y));
        }

        let mut turn = 1;

        while turn <= 200 {
            let direction = greedy_snake_move_barriers(current_snake.clone(), current_foods.clone(), barriers.clone());

            if access == 0 {
                if direction != -1 {
                    return -5;
                } else {
                    return 1;
                }
            }

            if direction < 0 || direction > 3 {
                return -4;
            }

            let new_snake = vec![
                current_snake[0] + (direction == 3) as i32 - (direction == 1) as i32,
                current_snake[1] + (direction == 0) as i32 - (direction == 2) as i32,
                current_snake[0],
                current_snake[1],
                current_snake[2],
                current_snake[3],
                current_snake[4],
                current_snake[5],
            ];

            if new_snake[0] < 1 || new_snake[0] > 8 || new_snake[1] < 1 || new_snake[1] > 8 {
                return -1;
            }

            if barriers_list.contains(&(new_snake[0], new_snake[1])) {
                return -2;
            }

            let mut ate_index = -1;
            for i in (0..current_foods.len()).step_by(2) {
                if new_snake[0] == current_foods[i] && new_snake[1] == current_foods[i + 1] {
                    ate_index = i as i32;
                    break;
                }
            }

            if ate_index != -1 {
                current_foods.remove(ate_index as usize);
                current_foods.remove(ate_index as usize);
                food_num -= 1;
            }

            if food_num == 0 {
                println!("Total turn: {}", turn);
                return turn; 
            }

            current_snake = new_snake;
            turn += 1;
        }

        -3
    }

    #[test]
    fn test_case_1() {
        assert!(greedy_snake_barriers_checker(
            vec![4, 4, 4, 3, 4, 2, 4, 1],
            1,
            vec![4, 5],
            vec![5, 4, 8, 8, 8, 7, 8, 6, 8, 5, 8, 4, 8, 3, 8, 2, 8, 1, 7, 8, 7, 7, 7, 6],
            1
        ) > 0);
    }
    #[test]
    fn test_case_2() {
        assert!(greedy_snake_barriers_checker(
            vec![1, 4, 1, 3, 1, 2, 1, 1],
            1,
            vec![5, 5],
            vec![2, 7, 2, 6, 3, 7, 3, 6, 4, 6, 5, 6, 6, 6, 7, 6, 4, 5, 4, 4, 4, 3, 5, 4],
            1
        ) > 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(greedy_snake_barriers_checker(
            vec![1, 4, 1, 3, 1, 2, 1, 1],
            1,
            vec![1, 7],
            vec![2, 7, 2, 6, 3, 7, 3, 6, 4, 7, 4, 6, 5, 7, 5, 6, 1, 6, 6, 6, 7, 6, 8, 6],
            0
        ), 1);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(greedy_snake_barriers_checker(
            vec![4, 4, 4, 3, 4, 2, 4, 1],
            1,
            vec![7, 5],
            vec![5, 4, 5, 5, 5, 6, 6, 4, 6, 5, 6, 6, 7, 4, 7, 6, 7, 7, 8, 4, 8, 5, 8, 6],
            0
        ), 1);
    }

    #[test]
    fn test_case_5() {
        assert!(greedy_snake_barriers_checker(
            vec![1, 1, 1, 2, 1, 3, 1, 4],
            1,
            vec![8, 8],
            vec![5, 5, 5, 6, 5, 7, 6, 5, 6, 6, 6, 7, 7, 5, 7, 6, 7, 7, 8, 5, 8, 6, 8, 7],
            1
        ) > 0);
    }
}