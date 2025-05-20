# Mew Language Syntax Specification

The Mew programming language is a cat-themed interpreted language with a syntax that combines familiar programming constructs with feline-inspired keywords and expressions.

## Comments

```
// Single line comments start with double forward slashes
```

## Variables and Constants

```
catst PI = 3.14159;      // Constant (cannot be reassigned)
catlt radius = 5;        // Let variable (block scoped)
catv message = "Hello";  // Var variable
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

#### For-In Loop (for arrays/objects)
```
fur (catlt index in array) {
    // code to execute for each index
}
```

#### For-Of Loop
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

### Arrow Functions
```
catlt functionName = (parameter1, parameter2) => {
    // function body
    return value;
};
```

## Output

```
purr("Hello, world!");  // Print to console
purr(expression);       // Print result of expression
```

## Importing

```
import { something } from "module";
```

## Error Handling

Try-catch blocks and error handling syntax may be available but aren't illustrated in the examples.

## Examples

### Hello World
```
purr("Hello, Mew world!");
```

### Function Example
```
cat calculateArea(radius) {
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