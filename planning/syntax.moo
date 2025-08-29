moo "Print statement" 

text name = "chicken"

num numero = 10

decimal numberdec = 21.5

boolean isHappy = true

coop 'text' granja = ["Chicken","cows","moooo?"]

granja.add("Hola")


if (name is chicken) {

    moo "The chicken name is chicken"

} else if (name is not chicken){

    moo "The chicken's name is not chicken"

}

else{

    moooo

}


while (numero less than 10){
    numero++
    moo "numero is:", numero

}

for cow in barn(10){
    moo "cow number",cow
}


farmfunction feedFarm(num n, coop 'num' farm){
        farm.add(n)
}





#comentarios

#*
multiline comments 
*#

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
