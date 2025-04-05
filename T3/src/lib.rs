use wasm_bindgen::prelude::*;
use std::collections::HashSet;

#[wasm_bindgen]
pub fn greedy_snake_step(arena: &[i32]) -> i32 {
    use std::collections::HashSet;

    let n = arena[0] as usize;
    let snake = &arena[1..9];
    let snake_num = arena[9] as usize;
    let other_snakes_len = snake_num * 8;
    let other_snakes_start = 10;
    let other_snakes_end = other_snakes_start + other_snakes_len;
    let other_snakes = &arena[other_snakes_start..other_snakes_end];
    let food_num = arena[other_snakes_end] as usize;
    let foods_start = other_snakes_end + 1;
    let foods_end = foods_start + food_num * 2;
    let foods = &arena[foods_start..foods_end];
    let round = arena[foods_end];


    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // 上, 左, 下, 右
    let mut best_score = i32::MIN;
    let mut best_dir = 0;

    let head = (snake[0], snake[1]);

    // 绝对碰撞集合：其他蛇现有123，自己蛇现有2
    let mut obstacles = HashSet::new();
    obstacles.insert((snake[2], snake[3])); 
    for snake_index in 0..snake_num {
        let start_index = snake_index * 8 as usize;
        for section_index in 0..3 {
            let x_index = start_index + section_index as usize * 2;
            let y_index = start_index + section_index as usize * 2 + 1;
            obstacles.insert((other_snakes[x_index], other_snakes[y_index]));
        }
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

            score += 10 / (dist_after + 1);
        }

            
        // 远离其他蛇头
        for i in 0..snake_num as usize {
            let other_head = (other_snakes[i * 8], other_snakes[i * 8 + 1]);
            let dist = ((other_head.0 - new_head.0).abs() + (other_head.1 - new_head.1).abs()) as i32;
            if dist <= 5 {
                score -= 20 / (dist + 1); // 太近的蛇头有风险
            }
        }

        println!("score:({},{})",dir_index,score);

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
    fn test_greedy_snake_step_two_snakes() {
        // 场地大小为 5×5，两条蛇，果子数量为 5
        let arena = vec![
            5, // 地图大小 n = 5
            4, 1, 3, 1, 2, 1, 1, 1, 
            1,
            2, 5, 3, 5, 4, 5, 5, 5, // 其他蛇 (1,1) (2,2) (3,3) (4,4)
            5, // 食物数量
            2, 3, 3, 2, 3, 4, 4, 3, 5, 3, // 食物 (1,2) (2,3) (3,4) (4,5) (5,1)
            11, // 当前轮数
        ];
        assert_eq!(greedy_snake_step(&arena), 0); // 预期方向：上
    }
}