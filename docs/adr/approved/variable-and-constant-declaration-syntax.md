# Variable & Constant Declaration

- Status: solved
- Deciders: Kane Petra, Wouter Pennings, Kenneth Luper, Vincent Kuilman
- Date: 08-04-2022
- Loop version: 0.2.0

Issue: https://looplang.atlassian.net/browse/LOOP-141

## **Context and Problem Statement**

Variables declarations are not consistent and are difficult to grasp for new users.

## **Decision Drivers**

- Consistency in the language is one of our philosophies
- Short but clear syntax

## **Considered Options**

1. Use an “infer” or “auto” type and require specification to declare variables
2. Use Golang style “:=” to declare & “=” to assign with an addition to allow type specification
3. Python style, as “IDENTIFIER = EXPRESSION” with no type specification allowed but only inferred

## **Decision Outcome**

Chosen option 2 because consistent with the least needed syntax while staying readable.

### **Positive Consequences**

- Syntax is consistent
- Easy to deduce types
- Possible to specify types

### **Negative Consequences**

- Syntax is not used by other scripting languages, increasing the barrier of entry slightly

## **Pros and Cons of the Options**

### Use an “infer” or “auto” type and require specification to declare variables

- Good, because you always specify a form of type
- Bad, because syntax is longer
- Bad, because syntax is less consistent

### Use Golang style “:=” to declare & “=” to assign with an addition to allow type specification

- Good, because the syntax is consistent
- Good, because types are clear
- Good, because syntax is shortened without losing information
- Bad, because its unique in scripting languages which might increase barrier of entry

### Python style, as “IDENTIFIER = EXPRESSION” with no type specification allowed but only inferred

- Good, because the syntax is short
- Good, because the syntax is consistent
- Bad, because the syntax is not clear whether you are declaring a variable or assigning to one
- Bad, because you cant specify types