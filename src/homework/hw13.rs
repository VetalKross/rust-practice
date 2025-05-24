// src/homework/hw13.rs

// Примітивне рішення для підрахунку зайнятої площі прямокутників

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point, // верхня ліва точка
    b: Point, // нижня права точка
}

// Функція рахує площу, яка покрита хоча б одним прямокутником
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // 1) знаходимо максимальні x та y
    let mut max_x = 0;
    let mut max_y = 0;
    for r in xs {
        if r.b.x > max_x { max_x = r.b.x; }
        if r.a.x > max_x { max_x = r.a.x; }
        if r.a.y > max_y { max_y = r.a.y; }
        if r.b.y > max_y { max_y = r.b.y; }
    }

    // 2) створюємо 2D-вектор (решітку) заповнений false
    let mut grid = Vec::new();
    for _ in 0..=max_y {
        grid.push(vec![false; (max_x + 1) as usize]);
    }

    // 3) заповнюємо клітинки для кожного прямокутника
    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                grid[y as usize][x as usize] = true;
            }
        }
    }

    // 4) рахуємо всі "зайняті" клітинки
    let mut area = 0;
    for row in &grid {
        for &cell in row {
            if cell {
                area += 1;
            }
        }
    }

    area // повертаємо результат
}

// Додаємо порожню точку входу, щоб можна було Cargo run без помилок
fn main() {
    // Приклад: рахуємо площу для тестових даних
    let rectangles = vec![
        Rectangle { a: Point { x: 2, y: 9 },  b: Point { x: 5,  y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 },  b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ];
    let occupied = area_occupied(&rectangles);
    println!("Occupied area = {}", occupied);
}

#[cfg(test)]
mod tests {
    use super::*;

    // тестові дані, як в умові
    fn test_data() -> Vec<Rectangle> {
        vec![
            Rectangle { a: Point { x: 2, y: 9 },  b: Point { x: 5,  y: 3 } },
            Rectangle { a: Point { x: 1, y: 8 },  b: Point { x: 11, y: 6 } },
            Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
        ]
    }

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let res = area_occupied(&data);
        assert_eq!(res, 60);
    }
}
