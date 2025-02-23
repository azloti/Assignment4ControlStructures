use serde::Deserialize;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, write};

const MAX_DAYS: u32 = 5;
const SHIFT_CAPACITY: usize = 2;

#[derive(Deserialize)]
struct EmployeeInput {
    name: String,
    preferences: HashMap<String, String>,
}

struct Employee {
    name: String,
    preferences: HashMap<String, String>,
    days_worked: u32,
    scheduled_days: HashSet<String>,
}

fn main() {
    // Read and parse input.json
    let input_data = read_to_string("input.json").expect("Failed to read input.json");
    let employee_inputs: Vec<EmployeeInput> =
        serde_json::from_str(&input_data).expect("Failed to parse input.json");

    // Build employees from input
    let mut employees: Vec<Employee> = employee_inputs
        .into_iter()
        .map(|emp| Employee {
            name: emp.name,
            preferences: emp.preferences,
            days_worked: 0,
            scheduled_days: HashSet::new(),
        })
        .collect();

    // Day and shift definitions
    let days = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sunday".to_string(),
    ];
    let shifts = vec!["morning".to_string(), "afternoon".to_string(), "evening".to_string()];

    let mut schedule: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for day in &days {
        let mut shift_map = HashMap::new();
        for shift in &shifts {
            shift_map.insert(shift.clone(), Vec::new());
        }
        schedule.insert(day.clone(), shift_map);
    }

    // Start the backtracking algorithm from day 0, shift 0, slot 0.
    if backtrack(0, 0, 0, &mut employees, &mut schedule, &days, &shifts) {
        let output =
            serde_json::to_string_pretty(&schedule).expect("Failed to serialize schedule");
        write("output.json", output).expect("Failed to write output.json");
        println!("Schedule generated in output.json");
    } else {
        eprintln!("No valid schedule could be generated.");
    }
}

fn backtrack(
    day_index: usize,
    shift_index: usize,
    slot_index: usize,
    employees: &mut Vec<Employee>,
    schedule: &mut HashMap<String, HashMap<String, Vec<String>>>,
    days: &Vec<String>,
    shifts: &Vec<String>,
) -> bool {
    if day_index == days.len() {
        return true;
    }

    let day = &days[day_index];
    let shift = &shifts[shift_index];

    if slot_index == SHIFT_CAPACITY {
        if shift_index < shifts.len() - 1 {
            return backtrack(day_index, shift_index + 1, 0, employees, schedule, days, shifts);
        } else {
            return backtrack(day_index + 1, 0, 0, employees, schedule, days, shifts);
        }
    }

    let mut eligible: Vec<usize> = employees
        .iter()
        .enumerate()
        .filter(|(_, emp)| emp.days_worked < MAX_DAYS && !emp.scheduled_days.contains(day))
        .map(|(i, _)| i)
        .collect();

    eligible.sort_by_key(|&i| {
        let emp = &employees[i];
        // A one-liner: preference matches get 0; non-matches get 1.
        if emp.preferences.get(day).map(|pref| pref == shift).unwrap_or(false) {
            0
        } else {
            1
        }
    });

    // Try assigning each eligible employee to the current slot.
    for i in eligible {
        // Assign employee to schedule[day][shift]
        schedule
            .get_mut(day)
            .unwrap()
            .get_mut(shift)
            .unwrap()
            .push(employees[i].name.clone());
        employees[i].days_worked += 1;
        employees[i].scheduled_days.insert(day.clone());

        // Recurse to assign the next slot.
        if backtrack(day_index, shift_index, slot_index + 1, employees, schedule, days, shifts) {
            return true;
        }

        // Undo the assignment if the recursive call failed.
        schedule.get_mut(day).unwrap().get_mut(shift).unwrap().pop();
        employees[i].days_worked -= 1;
        employees[i].scheduled_days.remove(day);
    }
    // If no eligible assignment leads to a solution, return false.
    false
}
