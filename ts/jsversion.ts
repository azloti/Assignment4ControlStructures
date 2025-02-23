import { readFileSync, writeFileSync } from "fs";

const Days = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
  "Sunday",
] as const;
const Shifts = ["morning", "afternoon", "evening"] as const;

// Read and parse input.json (expected to be in JSON format)
const inputData = readFileSync("input.json", "utf8");

// Parse employee info and add helper properties
const employees = (
  JSON.parse(inputData) as {
    name: string;
    preferences: { [day in (typeof Days)[number]]: (typeof Shifts)[number] };
  }[]
).map((emp) => ({
  name: emp.name,
  preferences: emp.preferences,
  daysWorked: 0,
  scheduledDays: new Set<(typeof Days)[number]>(),
}));

// Initialize schedule: each day maps to an object with shifts as keys and arrays for employee names
const schedule = {} as {
  [day in (typeof Days)[number]]: {
    [shift in (typeof Shifts)[number]]: string[]; // Employee names
  };
};
Days.forEach((day) => {
  schedule[day] = { morning: [], afternoon: [], evening: [] };
});

function backtrack(
  dayIndex: number, // Index of the current day
  shiftIndex: number, // Index of the current shift
  slotIndex: number // 0 or 1: the current shift is being filled
): boolean {
  // If we've assigned all days, we're done and have succeeded!
  if (dayIndex === Days.length) {
    return true;
  }

  const day = Days[dayIndex];
  const shift = Shifts[shiftIndex];

  if (slotIndex === 2) {
    // All slots filled for this shift
    if (shiftIndex < Shifts.length - 1) {
      // Next shift in the day
      return backtrack(dayIndex, shiftIndex + 1, 0);
    } else {
      // Next day
      return backtrack(dayIndex + 1, 0, 0);
    }
  }

  const eligible = employees.filter(
    // Max 5 days per employee, and not already scheduled on this day.
    (emp) => emp.daysWorked < 5 && !emp.scheduledDays.has(day)
  );

  // Sort eligible employees so that those who prefer this shift are tried first.
  eligible.sort((a, b) => {
    const aPref = a.preferences[day] === shift ? 0 : 1;
    const bPref = b.preferences[day] === shift ? 0 : 1;
    return aPref - bPref;
  });

  for (const emp of eligible) {
    // Assign this employee to the current slot.
    schedule[day][shift].push(emp.name);
    emp.daysWorked++;
    emp.scheduledDays.add(day);

    // Recurse to assign the next slot.
    if (backtrack(dayIndex, shiftIndex, slotIndex + 1)) {
      return true;
    }

    // Didn't work: Undo the assignment.
    schedule[day][shift].pop();
    emp.daysWorked--;
    emp.scheduledDays.delete(day);
  }

  // No valid employee found for this slot, try the next variant.
  return false;
}

if (backtrack(0, 0, 0)) {
  writeFileSync("output.json", JSON.stringify(schedule, null, 2), "utf8");
  console.log("Schedule generated in output.json");
} else {
  console.error("No valid schedule could be generated.");
}
