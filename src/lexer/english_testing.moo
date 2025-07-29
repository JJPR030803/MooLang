# Complex Farm Management System

# Nested data structures
coop 'text' farmNames = ["North", "South", "East", "West"]
coop 'coop' farmData = [
    ["cow", "chicken", "sheep"],
    ["pig", "duck", "goose"],
    ["horse", "donkey"],
    ["rabbit", "goat"]
]

# Nested function with complex logic
farmfunction processAnimalData(coop 'text' animals, num startCount) {
    num total = startCount
    coop 'text' processed = []

    farmfunction updateCount(text animal, num modifier) {
        total = total + modifier
        processed.plant(animal)
        moo "Updated count for", animal, "new total:", total
    }

    for animal in animals {
        if (animal is "cow") {
            updateCount(animal, 10)
        } else if (animal is "chicken") {
            updateCount(animal, 5)
        } else {
            updateCount(animal, 1)
        }
    }

    return processed
}

# Nested loops with conditional logic
farmfunction analyzeFarms(coop 'coop' allFarms) {
    num farmIndex = 0

    while (farmIndex < allFarms.collect()) {
        moo "Analyzing farm:", farmNames[farmIndex]
        coop 'text' currentFarm = allFarms[farmIndex]

        for animalIndex in barn(currentFarm.collect()) {
            text currentAnimal = currentFarm[animalIndex]

            if (currentAnimal is "cow") {
                for count in barn(5) {
                    moo "Checking cow health status:", count
                    if (count < 3) {
                        moo "Health check passed"
                    } else {
                        moo "Need attention"
                    }
                }
            } else if (currentAnimal is "chicken") {
                num eggCount = 0
                while (eggCount < 3) {
                    moo "Collecting eggs:", eggCount
                    eggCount++
                }
            }
        }
        farmIndex++
    }
}

# Complex recursive function
farmfunction calculateFarmValue(coop 'text' animals, num depth) {
    if (depth < 0) {
        return 0
    }

    num value = 0
    for animal in animals {
        if (animal is "cow") {
            value = value + 100
            value = value + calculateFarmValue(animals, depth < 1)
        } else if (animal is "chicken") {
            value = value + 10
        } else {
            value = value + 50
        }
    }
    return value
}

# Nested conditional with loop
farmfunction manageInventory(text farmName, coop 'text' inventory) {
    if (farmName is "North") {
        for item in inventory {
            if (item is "cow") {
                while (inventory.collect() < 5) {
                    inventory.add("cow")
                    moo "Added cow to North farm"
                }
            }
        }
    } else if (farmName is "South") {
        num chickenCount = 0
        while (chickenCount < 3) {
            for item in inventory {
                if (item is "chicken") {
                    moo "Found chicken in South farm"
                }
            }
            chickenCount++
        }
    }
}

# Test all the complex functionality
moo "Starting complex farm tests"

# Test nested data access
for farmIndex in barn(farmData.collect()) {
    coop 'text' currentFarm = farmData[farmIndex]
    moo "Farm", farmNames[farmIndex], "has", currentFarm.collect(), "animals"

    # Nested processing
    coop 'text' processedData = processAnimalData(currentFarm, 0)
    moo "Processed animals:", processedData

    # Calculate and verify farm value
    num farmValue = calculateFarmValue(currentFarm, 2)
    moo "Farm value:", farmValue

    # Manage inventory
    manageInventory(farmNames[farmIndex], currentFarm)
}

# Run complete farm analysis
analyzeFarms(farmData)

# Hashmap testing section
moo "Testing Hashmap Operations"

# Basic hashmap creation and operations
barn_map 'text':'num' animalInventory = {
    "cow": 5,
    "chicken": 15,
    "pig": 3
}

# Adding and updating entries
animalInventory.put("sheep", 10)
animalInventory.put("cow", 6)  # Updates existing value

# Accessing values
num cowCount = animalInventory.get("cow")
moo "Number of cows:", cowCount

# Checking existence
if (animalInventory.contains("chicken")) {
    moo "We have chickens!"
}

# Removing entries
animalInventory.remove("pig")

# Complex hashmap with nested structures
barn_map 'text':'coop' animalGroups = {
    "mammals": ["cow", "sheep", "pig"],
    "birds": ["chicken", "duck", "goose"]
}

# Hashmap with object values
barn_map 'text':'barn_map' animalDetails = {
    "cow": {
        "age": 5,
        "weight": 500,
        "color": "brown"
    },
    "chicken": {
        "age": 2,
        "weight": 2,
        "color": "white"
    }
}

# Function to process hashmap data
farmfunction processAnimalStats(barn_map 'text':'barn_map' details) {
    num totalAge = 0

    for animal in details {
        num age = details.get(animal).get("age")
        totalAge = totalAge + age
        moo animal, "is", age, "years old"
    }

    return totalAge
}

# Frequency counter using hashmap
farmfunction countAnimalTypes(coop 'text' animals) {
    barn_map 'text':'num' frequency = {}

    for animal in animals {
        if (frequency.contains(animal)) {
            num count = frequency.get(animal)
            frequency.put(animal, count + 1)
        } else {
            frequency.put(animal, 1)
        }
    }

    return frequency
}

# Testing complex operations
coop 'text' farmAnimals = ["cow", "chicken", "cow", "pig", "chicken", "chicken"]
barn_map 'text':'num' animalFrequency = countAnimalTypes(farmAnimals)

for animal in animalFrequency {
    moo animal, "appears", animalFrequency.get(animal), "times"
}

# Testing nested hashmaps
barn_map 'text':'barn_map' farmManagement = {
    "north_barn": {
        "capacity": 50,
        "current": 30,
        "animals": {
            "cow": 10,
            "pig": 20
        }
    },
    "south_barn": {
        "capacity": 40,
        "current": 25,
        "animals": {
            "chicken": 15,
            "sheep": 10
        }
    }
}

# Complex query on nested hashmaps
farmfunction checkBarnCapacity(barn_map 'text':'barn_map' barns) {
    for barnName in barns {
        barn_map 'text':'num' barnInfo = barns.get(barnName)
        num available = barnInfo.get("capacity") < barnInfo.get("current")
        moo barnName, "has", available, "spaces available"
    }
}

# Testing modulo and integer division operations
moo "Testing Modulo and Integer Division Operations"

farmfunction 'num' calculateRemainder(num total, num divisor) {
    num remainder = total % divisor
    moo total, "divided by", divisor, "has remainder:", remainder
    return remainder
}

farmfunction 'num' calculateIntegerDivision(num total, num divisor) {
    num result = total // divisor
    moo total, "integer divided by", divisor, "equals:", result
    return result
}

# Test with various numbers
num eggsTotal = 25
num basketSize = 6

# Calculate how many full baskets we can make
num fullBaskets = calculateIntegerDivision(eggsTotal, basketSize)
moo "We can fill", fullBaskets, "baskets"

# Calculate how many eggs will be left over
num leftoverEggs = calculateRemainder(eggsTotal, basketSize)
moo "We will have", leftoverEggs, "eggs left over"

# Test with different farm scenarios
coop 'num' animalCounts = [10, 15, 7, 22]
coop 'num' penSizes = [4, 3, 2]

for animalCount in animalCounts {
    for penSize in penSizes {
        num fullPens = animalCount // penSize
        num leftoverAnimals = animalCount % penSize
        moo animalCount, "animals in pens of", penSize, "gives", fullPens, "full pens with", leftoverAnimals, "animals left over"
    }
}
