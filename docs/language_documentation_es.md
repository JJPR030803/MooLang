# MooLang: Un Lenguaje de Programación Educativo

## Filosofía e Intención

MooLang es un lenguaje de programación educativo diseñado para cerrar la brecha entre los lenguajes de programación basados en bloques como Scratch y los lenguajes tradicionales basados en texto como Java, C++ y Python. El lenguaje tiene como objetivo:

1. **Proporcionar una transición fácil** de la programación visual a la programación basada en texto
2. **Mantener la simplicidad** mientras introduce conceptos importantes de programación
3. **Soportar múltiples idiomas naturales** (actualmente inglés con extensión .moo y español con extensión .muu)
4. **Enseñar conceptos de programación orientada a objetos** de manera accesible
5. **Utilizar palabras clave con temática de granja** para crear un entorno de aprendizaje divertido y atractivo

MooLang es más maduro que Scratch pero mantiene un nivel similar de accesibilidad como Python, lo que lo hace ideal para entornos educativos donde los estudiantes están listos para ir más allá de la programación basada en bloques pero pueden no estar listos para la complejidad de los lenguajes de programación tradicionales.

## Características del Lenguaje

### Extensiones de Archivo
- `.moo` - Versión en inglés
- `.muu` - Versión en español

Ambas versiones se mapean a los mismos tokens y funcionalidades subyacentes, difiriendo solo en las palabras clave y la sintaxis utilizada.

### Sintaxis Básica

#### Salida
```
moo "Hello, World!"  # Versión en inglés
muuu "Hola, Mundo!"  # Versión en español
```

#### Variables y Tipos de Datos

MooLang soporta varios tipos de datos básicos:

| Tipo | Inglés | Español | Ejemplo (Español) |
|------|---------|---------|------------------|
| Texto | `text` | `texto` | `texto nombre = "pollo"` |
| Entero | `num` | `numero` | `numero contador = 10` |
| Decimal | `dec` | `decimal` | `decimal precio = 21.5` |
| Booleano | `boolean` | `booleano` | `booleano estaFeliz = verdadero` |
| Arreglo | `coop` | `granja` | `granja 'texto' animales = ["vaca", "pollo"]` |
| Mapa | `barn_map` | `granero_mapa` | `granero_mapa 'texto':'numero' inventario = {"vaca": 5}` |

#### Estructuras de Control

##### Condicionales
```
# Español
si (nombre es "pollo") {
    muuu "El nombre es pollo"
} sino si (nombre no es "pollo") {
    muuu "El nombre no es pollo"
} sino {
    muuu "Algo más"
}

# Inglés
if (name is "chicken") {
    moo "The name is chicken"
} else if (name is not "chicken") {
    moo "The name is not chicken"
} else {
    moo "Something else"
}
```

##### Bucles
```
# Español - Bucle Mientras
numero contador = 0
mientras (contador < 10) {
    contador++
    muuu "Contador es:", contador
}

# Español - Bucle Para
para animal en animales {
    muuu "Animal:", animal
}

# Español - Bucle Para basado en Rango
para indice en granero(10) {
    muuu "Índice:", indice
}
```

#### Funciones
```
# Español
funciongranja 'numero' calcularTotal(numero a, numero b) {
    regresa a + b
}

# Inglés
farmfunction 'num' calculateTotal(num a, num b) {
    return a + b
}
```

### Colecciones

#### Arreglos (Granja/Coop)
```
# Español
granja 'texto' animales = ["vaca", "pollo", "cerdo"]
animales.agregar("oveja")     # Agregar al final
animales.plantar("cabra")     # Alternativa para agregar
numero tamaño = animales.contar()  # Obtener tamaño del arreglo
```

#### Mapas (Granero_mapa/Barn_map)
```
# Español
granero_mapa 'texto':'numero' inventario = {
    "vaca": 5,
    "pollo": 10
}

inventario.poner("oveja", 3)           # Agregar o actualizar entrada
numero contadorVacas = inventario.obtener("vaca") # Obtener valor
inventario.quitar("pollo")             # Eliminar entrada
booleano tieneCerdos = inventario.contiene("cerdo") # Verificar si existe la clave
```

### Operadores

| Operación | Inglés | Español |
|-----------|---------|---------|
| Adición | `+` | `+` |
| Sustracción | `-` | `-` |
| Multiplicación | `*` | `*` |
| División | `/` | `/` |
| División Entera | `//` | `//` |
| Módulo | `%` | `%` |
| Igualdad | `is` | `es` |
| Desigualdad | `is not` | `no es` |
| Menor que | `<` | `<` |
| Mayor que | `>` | `>` |
| Incremento | `++` | `++` |
| Decremento | `--` | `--` |

### Comentarios
```
# Comentario de una línea

#*
Comentario
de múltiples
líneas
*#
```

## Programa de Ejemplo

```
# Gestión de inventario de granja
granja 'texto' animales = ["vaca", "pollo", "cerdo"]
granja 'numero' cantidades = [5, 10, 3]

funciongranja agregarNuevoAnimal(texto animal, numero cantidad) {
    animales.plantar(animal)
    cantidades.plantar(cantidad)
    muuu "Agregado", cantidad, animal, "a la granja"
}

funciongranja mostrarInventario() {
    para indice en granero(animales.contar()) {
        muuu animales[indice], "cantidad:", cantidades[indice]
    }
}

# Usando las funciones
agregarNuevoAnimal("oveja", 4)
muuu "Inventario actual de la granja:"
mostrarInventario()

# Calcular total de animales
numero total = 0
para cantidad en cantidades {
    total = total + cantidad
}
muuu "Total de animales en la granja:", total
```

## Conclusión

MooLang proporciona una introducción suave a la programación basada en texto mientras mantiene un tema divertido y atractivo. Al soportar versiones tanto en inglés como en español con funcionalidad idéntica, hace que la programación sea más accesible para una audiencia más amplia de estudiantes. El diseño del lenguaje se centra en la legibilidad y la sintaxis clara, convirtiéndolo en un paso ideal para estudiantes que están haciendo la transición de la programación basada en bloques a lenguajes de programación más tradicionales.
