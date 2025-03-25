use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: &[i32], fruit: &[i32]) -> i32 {
    let solutions = [0, 1, 2, 3];
    let mut default = -1; 
    for solution in solutions {
        let new_snake = [
            snake[0] + i32::from(solution == 3) - i32::from(solution == 1),
            snake[1] + i32::from(solution == 0) - i32::from(solution == 2),
            snake[0],
            snake[1],
            snake[2],
            snake[3],
            snake[4],
            snake[5],
        ];

        // 超界
        if new_snake[0] < 1 || new_snake[0] > 8 || new_snake[1] < 1 || new_snake[1] > 8 {
            continue;
        }

        // 头身碰撞
        if (new_snake[0] == new_snake[4] && new_snake[1] == new_snake[5]) || (new_snake[0] == new_snake[6] && new_snake[1] == new_snake[7]) {
            continue;
        }
        default = solution;

        // 是否靠近食物
        if (snake[0] > fruit[0] && solution == 1) || (snake[0] < fruit[0] && solution == 3) || (snake[1] > fruit[1] && solution == 2) || (snake[1] < fruit[1] && solution == 0) {
            return solution;
        } 
    }
    return default;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greedy_snake_move_test1() {
        let snake = [3,2,3,3,3,4,2,4];
        let fruit = [8,8];
        assert_eq!(greedy_snake_move(&snake, &fruit), 3);
    }

    #[test]
    fn greedy_snake_move_test2() {
        let snake = [1,1,1,2,1,3,1,4];
        let fruit = [1,8];
        assert_eq!(greedy_snake_move(&snake, &fruit), 3);
    }

    #[test]
    fn greedy_snake_move_test3() {
        let snake = [2,2,2,3,2,4,2,5];
        let fruit = [2,1];
        assert_eq!(greedy_snake_move(&snake, &fruit), 2);
    }

    #[test]
    fn greedy_snake_move_test4() {
        let snake = [4,4,4,5,5,5,5,4];
        let fruit = [5,4];
        assert_eq!(greedy_snake_move(&snake, &fruit), 3);
    }

    #[test]
    fn greedy_snake_move_test5() {
        let snake = [4,4,4,5,5,5,5,4];
        let fruit = [4,5];
        assert_eq!(greedy_snake_move(&snake, &fruit), 3);
    }
}