# MooLang: An Educational Programming Language

## Philosophy and Intent

MooLang is an educational programming language designed to bridge the gap between block-based programming languages like Scratch and more traditional text-based languages like Java, C++, and Python. The language aims to:

1. **Provide an easy transition** from visual programming to text-based programming
2. **Maintain simplicity** while introducing important programming concepts
3. **Support multiple natural languages** (currently English with .moo extension and Spanish with .muu extension)
4. **Teach object-oriented programming** concepts in an approachable way
5. **Use farm-themed keywords** to create a fun, engaging learning environment

MooLang is more mature than Scratch but maintains a similar level of accessibility as Python, making it ideal for educational settings where students are ready to move beyond block-based programming but may not be ready for the complexity of traditional programming languages.

## Language Features

### File Extensions
- `.moo` - English version
- `.muu` - Spanish version

Both versions map to the same underlying tokens and functionality, differing only in the keywords and syntax used.

### Basic Syntax

#### Output
```
moo "Hello, World!"  # English version
muuu "Hola, Mundo!"  # Spanish version
```

#### Variables and Data Types

MooLang supports several basic data types:

| Type | English | Spanish | Example (English) |
|------|---------|---------|------------------|
| Text | `text` | `texto` | `text name = "chicken"` |
| Integer | `num` | `numero` | `num count = 10` |
| Decimal | `dec` | `decimal` | `dec price = 21.5` |
| Boolean | `boolean` | `booleano` | `boolean isHappy = true` |
| Array | `coop` | `granja` | `coop 'text' animals = ["cow", "chicken"]` |
| Map | `barn_map` | `granero_mapa` | `barn_map 'text':'num' inventory = {"cow": 5}` |

#### Control Structures

##### Conditionals
```
# English
if (name is "chicken") {
    moo "The name is chicken"
} else if (name is not "chicken") {
    moo "The name is not chicken"
} else {
    moo "Something else"
}

# Spanish
si (nombre es "pollo") {
    muuu "El nombre es pollo"
} sino si (nombre no es "pollo") {
    muuu "El nombre no es pollo"
} sino {
    muuu "Algo más"
}
```

##### Loops
```
# English - While Loop
num count = 0
while (count < 10) {
    count++
    moo "Count is:", count
}

# English - For Loop
for animal in animals {
    moo "Animal:", animal
}

# English - Range-based For Loop
for index in barn(10) {
    moo "Index:", index
}
```

#### Functions
```
# English
farmfunction 'num' calculateTotal(num a, num b) {
    return a + b
}

# Spanish
funciongranja 'numero' calcularTotal(numero a, numero b) {
    return a + b
}
```

### Collections

#### Arrays (Coop/Granja)
```
# English
coop 'text' animals = ["cow", "chicken", "pig"]
animals.add("sheep")     # Add to the end
animals.plant("goat")    # Alternative to add
num size = animals.collect()  # Get array size
```

#### Maps (Barn_map/Granero_mapa)
```
# English
barn_map 'text':'num' inventory = {
    "cow": 5,
    "chicken": 10
}

inventory.put("sheep", 3)           # Add or update entry
num cowCount = inventory.get("cow") # Get value
inventory.remove("chicken")         # Remove entry
boolean hasPigs = inventory.contains("pig") # Check if key exists
```

### Operators

| Operation | English | Spanish |
|-----------|---------|---------|
| Addition | `+` | `+` |
| Subtraction | `-` | `-` |
| Multiplication | `*` | `*` |
| Division | `/` | `/` |
| Integer Division | `//` | `//` |
| Modulo | `%` | `%` |
| Equality | `is` | `es` |
| Inequality | `is not` | `no es` |
| Less than | `<` | `<` |
| Greater than | `>` | `>` |
| Increment | `++` | `++` |
| Decrement | `--` | `--` |

### Comments
```
# Single line comment

#*
Multi-line
comment
*#
```

## Example Program

```
# Farm inventory management
coop 'text' animals = ["cow", "chicken", "pig"]
coop 'num' counts = [5, 10, 3]

farmfunction addNewAnimal(text animal, num count) {
    animals.plant(animal)
    counts.plant(count)
    moo "Added", count, animal, "to the farm"
}

farmfunction showInventory() {
    for index in barn(animals.collect()) {
        moo animals[index], "count:", counts[index]
    }
}

# Using the functions
addNewAnimal("sheep", 4)
moo "Current farm inventory:"
showInventory()

# Calculate total animals
num total = 0
for count in counts {
    total = total + count
}
moo "Total animals in farm:", total
```

## Conclusion

MooLang provides a gentle introduction to text-based programming while maintaining a fun, engaging theme. By supporting both English and Spanish versions with identical functionality, it makes programming more accessible to a wider audience of learners. The language's design focuses on readability and clear syntax, making it an ideal stepping stone for students transitioning from block-based programming to more traditional programming languages.
