type Minutes = u16;

struct Meal {
    name: String,
    cuisine: String,
    ingredients: Vec<String>,
    difficulty: u8,
    time: Minutes,
    times_cooked: u8
}

// TODO: Create way of reading in file, de-serializing into Meal
//       objects.

#[cfg(test)]
mod tests {
    use crate::Meal;

    #[test]
    fn test_create_meal() {
        let freya_meal = Meal{
            name: "Freya's Turkey Sandwich".to_string(),
            cuisine: "Feline".to_string(),
            ingredients: vec!["homemade bread".to_string(), "turkey".to_string()],
            difficulty: 1,
            time: 5,
            times_cooked: 1,
        };
        assert_eq!(freya_meal.name, "Freya's Turkey Sandwich".to_string());
        assert_eq!(freya_meal.cuisine, "Feline".to_string());
        assert_eq!(freya_meal.ingredients, vec!["homemade bread".to_string(), "turkey".to_string()]);
        assert_eq!(freya_meal.difficulty, 1);
        assert_eq!(freya_meal.time, 5);
        assert_eq!(freya_meal.times_cooked, 1);
    }

    #[test]
    fn test_create_update_meal() {
        let freya_meal = Meal{
            name: "Freya's Turkey Sandwich".to_string(),
            cuisine: "Feline".to_string(),
            ingredients: vec!["homemade bread".to_string(), "turkey".to_string()],
            difficulty: 1,
            time: 5,
            times_cooked: 1,
        };
        
        // Freya's meal now belongs to Camryn.
        let mut camryn_meal = freya_meal;
        camryn_meal.name = "Toilet Mochi".to_string();
        camryn_meal.cuisine = "Human".to_string();
        camryn_meal.ingredients = vec!["toilet".to_string(), "ice cream mochi".to_string()];
        camryn_meal.difficulty = 1;
        camryn_meal.time = 2;
        camryn_meal.times_cooked = 10;

        assert_ne!(camryn_meal.name, "Freya's Turkey Sandwich".to_string());
        assert_eq!(camryn_meal.name, "Toilet Mochi".to_string());
        assert_ne!(camryn_meal.cuisine, "Feline".to_string());
        assert_eq!(camryn_meal.cuisine, "Human".to_string());
        assert_ne!(camryn_meal.ingredients, vec!["homemade bread".to_string(), "turkey".to_string()]);
        assert_eq!(camryn_meal.ingredients, vec!["toilet".to_string(), "ice cream mochi".to_string()]);
        assert_eq!(camryn_meal.difficulty, 1);
        assert_ne!(camryn_meal.time, 5);
        assert_eq!(camryn_meal.time, 2); 
        assert_ne!(camryn_meal.times_cooked, 1);
        assert_eq!(camryn_meal.times_cooked, 10);
    }

    // TODO: Add functions to read from files
}
