use wasm_bindgen::prelude::*;
use std::collections::HashSet;

#[wasm_bindgen]
pub fn greedy_snake_step(
    n: i32,
    snake: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    food_num: i32,
    foods: &[i32],
    round: i32,
) -> i32 {
    let n = n as usize;
    let snake_num = snake_num as usize;
    let food_num = food_num as usize;

    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // 上, 左, 下, 右
    let mut best_score = i32::MIN;
    let mut best_dir = 0;

    let head = (snake[0], snake[1]);

    if snake == &[-1, -1, -1, -1, -1, -1, -1, -1] {
        return 0; // 蛇已死，返回默认方向（向上）
    }

    // 绝对碰撞集合：所有蛇现有123
    let mut obstacles = HashSet::new();
    obstacles.insert((snake[0], snake[1])); 
    obstacles.insert((snake[2], snake[3])); 
    obstacles.insert((snake[4], snake[5])); 
    for snake_index in 0..snake_num {
        let start_index = snake_index * 8 as usize;
        for section_index in 0..3 {
            let x_index = start_index + section_index as usize * 2;
            let y_index = start_index + section_index as usize * 2 + 1;
            obstacles.insert((other_snakes[x_index], other_snakes[y_index]));
        }
    }

    // 计算新蛇头位置是否处于其他蛇头的潜在移动范围内
    fn calculate_risk(new_head: (i32, i32), other_snakes: &[i32], snake_num: usize, n: usize,obstacles:&HashSet<(i32,i32)>) -> i32 {
        let mut risk = 0;
        let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // 上, 左, 下, 右

        for i in 0..snake_num {
            let other_head = (other_snakes[i * 8], other_snakes[i * 8 + 1]);
            let mut possible_moves = 0;

            for &(dx, dy) in &directions {
                let potential_position = (other_head.0 + dx, other_head.1 + dy);
                if potential_position.0 >= 1 && potential_position.0 <= n as i32 &&
                   potential_position.1 >= 1 && potential_position.1 <= n as i32 &&
                   !obstacles.contains(&potential_position){
                    possible_moves += 1;
                    if potential_position == new_head {
                        risk -= 100; 
                    }
                }
            }

            // 如果其他蛇有多个可能的移动方向，减分应该更少
            if possible_moves > 1 {
                risk /= possible_moves;
            }
        }

        risk
    }

    for (dir_index, (dx, dy)) in directions.iter().enumerate() {
        let new_head = (head.0 + dx, head.1 + dy);
        let mut score = 0;

        // 绝对碰撞：边界
        if new_head.0 < 1 || new_head.1 < 1 || new_head.0 > n as i32 || new_head.1 > n as i32 {
            score -= 100;
        }

        // 绝对碰撞：自己或其他蛇身体碰撞
        if obstacles.contains(&new_head) {
            score -= 100;
        }

        // 食物接近加分（若此方向更接近某个食物，则加分；如果有较近的食物，再额外加分）
        for i in (0..foods.len()).step_by(2) {
            let food = (foods[i], foods[i + 1]);
            let dist_before = (food.0 - head.0).abs() + (food.1 - head.1).abs();
            let dist_after = (food.0 - new_head.0).abs() + (food.1 - new_head.1).abs();

            if dist_after < dist_before {
                score += 5;
            }

            score += 30 / (dist_after + 1);
        }

        // 远离其他蛇头
        for i in 0..snake_num as usize {
            let other_head = (other_snakes[i * 8], other_snakes[i * 8 + 1]);
            let dist = ((other_head.0 - new_head.0).abs() + (other_head.1 - new_head.1).abs()) as i32;
            if dist <= 5 {
                score -= 20 / (dist + 1); // 太近的蛇头有风险
            }
        }

        // 相对碰撞：计算新蛇头位置的风险
        score += calculate_risk(new_head, other_snakes, snake_num, n,&obstacles);

        println!("score:({},{})", dir_index, score);

        if score > best_score {
            best_score = score;
            best_dir = dir_index as i32;
        }
    }

    best_dir
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_greedy_snake_step_two_snakes_1() {
        let n = 5;
        let snake = [3, 2, 2, 2, 1, 2, 1, 3]; 
        let snake_num = 1;
        let other_snakes = [2, 3, 2, 4, 3, 4, 4, 4]; 
        let food_num = 5;
        let foods = [1, 1, 2, 5, 3, 5, 4, 5, 5, 2]; 
        let round = 11;

        let direction = greedy_snake_step(n, &snake, snake_num, &other_snakes, food_num, &foods, round);
        assert_eq!(direction, 3); 
    }

    #[test]
    fn test_greedy_snake_step_two_snakes_2() {

        let n = 5;
        let snake = [2,1,3,1,4,1,5,1]; // 自己的蛇
        let snake_num = 1;
        let other_snakes = [2,3,1,3,1,4,1,5]; // 其他蛇
        let food_num = 5;
        let foods = [1,1,2,4,2,5,3,5,4,5]; 
        let round = 10;

        let direction = greedy_snake_step(n, &snake, snake_num, &other_snakes, food_num, &foods, round);
        assert_eq!(direction, 1); // 预期方向：左
    }

    #[test]
    fn test_greedy_snake_step_two_snakes_3() {
        // 场地大小为 5×5，两条蛇，果子数量为 5
        let n = 5;
        let snake = [2,1,3,1,4,1,5,1]; // 自己的蛇
        let snake_num = 1;
        let other_snakes = [1,2,1,3,1,4,1,5]; // 其他蛇
        let food_num = 5;
        let foods = [1,1,2,4,2,5,3,5,4,5];
        let round = 12;

        let direction = greedy_snake_step(n, &snake, snake_num, &other_snakes, food_num, &foods, round);
        assert_eq!(direction, 0); // 预期方向：上
    }

    #[test]
    fn test_greedy_snake_step_two_snakes_4() {
        let n = 5;
        let snake = [2,1,3,1,4,1,5,1]; // 自己的蛇
        let snake_num = 1;
        let other_snakes = [1,2,2,2,3,2,4,2]; // 其他蛇
        let food_num = 5;
        let foods = [1,1,2,4,2,5,3,5,4,5]; 
        let round = 10;
        let direction = greedy_snake_step(n, &snake, snake_num, &other_snakes, food_num, &foods, round);
        assert_eq!(direction, 1); // 预期方向：左
    }

    #[test]
    fn test_greedy_snake_step_two_snakes_5() {
        let n = 5;
        let snake = [3,2,3,3,2,3,2,2]; // 自己的蛇
        let snake_num = 1;
        let other_snakes = [4,4,3,4,2,4,1,4]; // 其他蛇
        let food_num = 5;
        let foods = [3,1,4,2,5,3,5,4,4,1]; 
        let round = 14;

        let direction = greedy_snake_step(n, &snake, snake_num, &other_snakes, food_num, &foods, round);
        assert_eq!(direction, 3); // 预期方向：右
    }
}