use super::intcode::intcode::{call_intcode, State};
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Recipe {
    ingredients: Vec<Ingredient>,
    result: Ingredient,
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    name: String,
    count: Cell<i32>,
}

#[aoc_generator(day14)]
fn parse_input_day12<'a>(input: &str) -> Vec<Recipe> {
    let re = Regex::new(r"(?P<ingredients>.*) => (?P<result>.*)").unwrap();

    let mut recipies: Vec<Recipe> = input
        .lines()
        .map(|recipe| {
            let components = &re.captures(recipe).unwrap();
            let ingredients: Vec<Ingredient> = components
                .name("ingredients")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|ingredient| {
                    let split: Vec<&str> = ingredient.split(" ").collect();
                    let ingredient_struct = Ingredient {
                        count: Cell::new(split[0].parse::<i32>().unwrap()),
                        name: split[1].to_string(),
                    };
                    ingredient_struct
                })
                .clone()
                .collect();

            let result: Vec<&str> = components
                .name("result")
                .unwrap()
                .as_str()
                .split(" ")
                .collect();

            Recipe {
                ingredients: ingredients,
                result: Ingredient {
                    count: Cell::new(result[0].parse::<i32>().unwrap()),
                    name: result[1].to_string(),
                },
            }
        })
        .collect();

    for recipe in recipies.iter() {
        let mut numbers: Vec<i32> = Vec::new();

        for ingredient in recipe.ingredients.clone() {
            numbers.push(ingredient.count.get());
        }
        numbers.push(recipe.result.count.get());

        let gcd_count = gcd(numbers);

        for ingredient in recipe.ingredients.iter() {
            ingredient.count.set(ingredient.count.get() / gcd_count);
        }
        recipe
            .result
            .count
            .set(recipe.result.count.get() / gcd_count);
    }

    recipies
}

pub fn gcd(numbers: Vec<i32>) -> i32 {
    let mut lcd = *numbers.iter().max().unwrap();

    for i in 0..(numbers.len()) {
        let mut x: i32 = lcd;
        let mut y: i32 = numbers[i];

        lcd = {
            while y != 0 {
                let temp = x % y;
                x = y;
                y = temp;
            }

            x
        }
    }

    lcd
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Recipe]) -> i128 {
    println!("{:?}", input[0]);
    let mut recipe_book = HashMap::new();

    for recipe in input {
        recipe_book.insert(
            recipe.result.name.clone(),
            (recipe.result.count.clone(), recipe.ingredients.clone()),
        );
    }

    println!("{:?}", get_ingredient_cost(&recipe_book, "FUEL"));

    1
}

fn get_ingredient_cost(
    recipe_book: &HashMap<String, (Cell<i32>, Vec<Ingredient>)>,
    ingredient_requested: &str,
    amount_needed: i32,
) -> i32 {
    let mut total_cost = 0;
    let this_recipe = recipe_book.get(ingredient_requested).unwrap();

    for ingredient in this_recipe.1.iter() {
        if ingredient.name == "ORE" {
            return ingredient.count.get() * this_recipe.0.get();
        }

        total_cost += get_ingredient_cost(recipe_book, &ingredient.name, &ingredient.count);
    }
    println!(
        "{} {} {}",
        ingredient_requested,
        total_cost,
        this_recipe.0.get()
    );
    total_cost * this_recipe.0.get()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Recipe]) -> i128 {
    1
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
    use super::gcd;

    #[test]
    fn sample1411() {
        assert_eq!(6, gcd(vec![12, 18, 30, 24]));
    }
}
