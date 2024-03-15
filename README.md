# PDM
## Tipos de datos
Los tipos de datos que contiene el lenguaje son: enteros, flotantes, booleanos, char y strings
- Enteros(int)
- Flotantes(float)
- Booleanos(bool)
- Caracteres(char)
- Strings(string) 

A pesar de ser fuertemente tipado, se usa la palabra "let" para definir cualquiera de estos
tipos de datos, puesto que el lenguaje infiere de cual se trata. Excepto en el caso de
string y char, si se quiere usar un caracter, se debe hacer uso explícitamente de 'c'.

## Sentencias de control
Cuenta con las siguientes sentencias de control de flujo:
- if
- else
- else if

Así mismo, tambien cuenta con los siguientes loops: 
- For
- While

## Funciones
La palabra reservada para las funciones es: 
- func
A menos que se específique un tipo de dato, todas las variables retornan void, para
especificar un retorno distinto, hace hace uso de: 
```
func ejemplo() -> int {}
```

## Operaciones
Puede realizar las operaciones aritméticas más comunes, como lo son: 
- Suma (+)
- Resta (-)
- Multiplicación (*)
- División (/)
- Módulo (%)

## Comentarios
Al igual que en otros lenguajes, los comentarios en una linea se hacen con // y
en multilinea se hace uso de /* */

## Fase del lexer
El lexer considera 6 tipos de tokens generales: 
- Identifiers
- Operators
- Constants
- Keyword
- Literals
- Punctuations
- Special chars

During this lexing phase only the next error is handled: 
- Unexpected token

# Code example

```
func main () {
    let hola = 45; 
    let HolaMundo = "Hola mundo"; 


//Operaciones
    let numero1 = 35;
    let numero2 = 33.4;
    let numero3 = 1_000_000;

    let suma = numero1 + numero3;
    let resta = numero3 - numero1;
    let division = numero3 / numero1;
    let multiplicacion = numero1 * numero3;
    let modulo = numero1 % numero3;

    if (resta > 0){
        printf("El número es positivo");
    }
    else if (resta < 0){
        printf("El número es negativo");
    }
    else {
        printf("El número es 0");
    }
}

    func printf(string: hola) {
        //Aquí se debería imprimir algo
    }

```

