# Recursive Return Type Checking

- Status: solved
- Deciders: Kane Petra, Wouter Pennings
- Date: 08-04-2022
- Loop version: 0.2.0

Issue: https://looplang.atlassian.net/browse/LOOP-145

## **Context and Problem Statement**

We are using the STD library from D lang to provide printing to stdout functionality. We can not ensure return types due to it being evaluated at runtime and not during compilation. This causes us to manually specify which expressions that contain an STD function should not have a “writeln” put around it if not manually specified.

## **Decision Drivers**

- It needs to be expandable without manually specifying function names to not put “writeln” around

## **Considered Options**

1. A custom standard library abstracted around with Loop code ensuring we know return types
2. We can still keep a list of functions without a known return type from D
3. Analyze the STD library during CICD to ensure return types
4. We can use the D standard library and write an abstraction using Loop around it

## **Decision Outcome**

Chosen option 1 because its the safest, fastest and easiest solution.

Option 2 was not chosen because D can change its std library changing return types and in addition we do not want to hardcode values

Option 3 was not chosen because its a complex solution with not much added value over the other options

Option 4 was not chosen because its a complex solution with not much added value over the other options

### **Positive Consequences**

- No static values in the compiler
- Everything is maintained by Loop developers ensuring the same philosophy

### **Negative Consequences**

- Extra maintenance vs setting up automation

## **Pros and Cons of the Options**

### A custom standard library abstracted around with Loop code ensuring we know return types

Example

```rust
#[no_mangle]
extern "C" pub fn println(string: *const c_char) {
	let value = CStr::from_ptr(c_char).to_str();
	println!("{}", value);
}
```

```jsx
// Example code subject to change

// Imports the std.dll (or std.so on unix) using FFI
import std;

println("Hello World!");
```

- Good, because we can ensure what everything returns
- Good, because its written by us meaning we know more about it
- Bad, because it causes extra maintenance (fixing bugs, adding new features etc)

### We can still keep a list of functions without a known return type from D

- Bad, because its a static list that needs to be manually updated in the compiler
- Bad, because return types can change in the D STD library

### Analyze the STD library during CICD to ensure return types

- Good, because its a one time setup to automate
- Bad, because we need to parse the D STD to find return types
- Bad, because it increases CICD time
- Bad, because its really complex
- Bad, because a manual trigger is needed for every time the STD updates

### We can use the D standard library and write an abstraction using Loop around it

- Good, because we don’t need to implement a standard library
- Good, because we can manually designate return types and update our STD if the STD of D changes without needing to update the compiler itself
- Bad, because of many duplicate code
- Bad, because of complexity
- Bad, because when writing an abstraction we can just as well write the implementation ourselves as well and maintain it ourselves in the philosophy of Loop
- Bad, because we need to convert D types to Loop types adding more complexity