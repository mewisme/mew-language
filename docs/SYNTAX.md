# Mew Language Syntax Specification

The Mew programming language is a cat-themed interpreted language with a syntax that combines familiar programming constructs with feline-inspired keywords and expressions.

## Comments

```
// Single line comments start with double forward slashes
```

## Variables and Constants

```
catst PI = Mewth.PI;      // Constant (cannot be reassigned)
catlt name = "Whiskers"; // Let variable (block scoped)
catv counter = 0;        // Var variable
```

## Data Types

### Primitive Types
- Numbers: `42`, `3.14159`
- Strings: `"Hello, world!"`
- Booleans: `true`, `false`
- Special values: `null`, `undefined`, `NaN`, `Infinity`

### Composite Types
- Arrays: `["apple", "banana", "cherry"]`
- Objects: `{ name: "Whiskers", age: 3, color: "orange" }`

## Operators

### Arithmetic Operators
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Modulo: `%`
- Increment: `++`
- Decrement: `--`

### Comparison Operators
- Equal: `==`
- Not Equal: `!=`
- Less Than: `<`
- Less Than or Equal: `<=`
- Greater Than: `>`
- Greater Than or Equal: `>=`

### Logical Operators
- And: `&&`
- Or: `||`
- Not: `!`

### Assignment Operators
- Assignment: `=`
- Add and assign: `+=`
- Subtract and assign: `-=`
- Multiply and assign: `*=`
- Divide and assign: `/=`
- Modulo and assign: `%=`

## Control Flow

### Conditional Statements

#### If-Else Statements
```
meow? (condition) {
    // code if condition is true
} meowse? (another_condition) {
    // code if another_condition is true
} hiss {
    // code if all conditions are false
}
```

#### Ternary Operator
```
condition ? true_expression : false_expression
```

#### Switch Statements
```
catwalk(expression) {
    claw value1:
        // code to execute
        clawt;
    claw value2:
        // code to execute
        clawt;
    default:
        // default code
}
```

### Loops

#### While Loop
```
mewhile (condition) {
    // code to execute while condition is true
}
```

#### Do-While Loop
```
domeow {
    // code to execute at least once
} mewhile (condition);
```

#### For Loop
```
fur (catlt i = 0; i < 10; i++) {
    // code to execute in loop
}
```

#### For-In Loop (for objects)
```
fur (catlt key in object) {
    // code to execute for each key
}
```

#### For-Of Loop (for arrays)
```
fur (catlt value of array) {
    // code to execute for each value
}
```

### Control Flow Keywords
- `break` - Exit a loop
- `continue` - Skip to the next iteration of a loop
- `return` - Return from a function

## Functions

### Function Declaration
```
cat functionName(parameter1, parameter2) {
    // function body
    return value;
}
```

### Function Expression (anonymous function)
```
catlt functionName = cat(parameter1, parameter2) {
    // function body
    return value;
};
```

## Output and Debugging

```
purr("Hello, world!");  // Print to console
purr(expression);       // Print result of expression
```

## Built-in Objects and Methods

### MewJ (JSON)
```
// Parse JSON string to object
catlt obj = MewJ.sniff(jsonString);

// Convert object to JSON string
catlt jsonStr = MewJ.mewify(obj);
catlt prettyJson = MewJ.mewify(obj, 2); // With 2-space indentation
```

### Mewth (Math)
```
Mewth.pounce(3.7);       // Floor (3)
Mewth.leap(3.2);         // Ceiling (4)
Mewth.curl(4.5);         // Round (5)
Mewth.lick(-9);          // Absolute value (9)
Mewth.alpha(1, 5, 3);    // Maximum value (5)
Mewth.kitten(1, 5, 3);   // Minimum value (1)
Mewth.chase();           // Random number between 0-1
Mewth.dig(25);           // Square root (5)
Mewth.scratch(2, 3);     // Power (2^3 = 8)
Mewth.tailDirection(-5); // Sign (-1, 0, or 1)
```

### CatTime (Date)
```
// Get current timestamp in milliseconds
CatTime.now();

// Create a new date object
catlt date = CatTime.wakeUp();

// Get date components
CatTime.fullYear(date);   // Year (e.g., 2023)
CatTime.month(date);      // Month (0-11)
CatTime.day(date);        // Day of month (1-31)
CatTime.weekday(date);    // Day of week (0-6, Sunday is 0)

// Get time components
CatTime.hours(date);      // Hours (0-23)
CatTime.minutes(date);    // Minutes (0-59)
CatTime.seconds(date);    // Seconds (0-59)
CatTime.milliseconds(date); // Milliseconds (0-999)

// Convert to string
CatTime.toMeow(date);     // Date as string
```

## String Methods and Properties

```
// String length
catlt str = "Meow!";
catlt length = str.length;

// String methods are also available
str.charAt(0);           // "M"
str.substring(1, 3);     // "eo"
```

## Array Methods and Properties

```
// Array length
catlt arr = ["apple", "banana", "cherry"];
catlt size = arr.length;

// Accessing array elements
catlt firstItem = arr[0];         // "apple"
catlt lastItem = arr[arr.length - 1]; // "cherry"
```

## Examples

### Hello World
```
purr("Hello, Mew world!");
```

### Function Example
```
cat calculateArea(radius) {
    catst PI = 3.14159;
    return PI * radius * radius;
}
```

### Loop Example
```
catlt fruits = ["apple", "banana", "cherry"];
fur (catlt i = 0; i < fruits.length; i++) {
    purr(fruits[i]);
}
```

### Object Example
```
catlt cat = {
    name: "Whiskers",
    age: 3,
    colors: ["orange", "white"],
    purr: cat() {
        return "Purrrrrr!";
    }
};
``` 