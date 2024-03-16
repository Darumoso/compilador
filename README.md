# PDM
## Intallation

### Dependencies
In case rust isn't installed, it can be installed with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building 
First clone the repo: 

```
git clone https://github.com/Darumoso/compilador && cd compilador
```

Then build with:
```
cargo build --release
```

### Runing the lexer
Once the project is succesfully builded, a binary can be found at:
```
cd target/release
```
Then make it executable
```
chmod +x ./compilador
```
To run it, enter: 
```
./compilador (file
)``

An output file will be generated in the directory where the command was
executed.

## Data types
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

It only supports the next loop statement: 
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
Al igual que en otros lenguajes, los comentarios en una linea se hacen con //

## Fase del lexer
El lexer considera 7 tipos de tokens generales: 
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
    let number = 45; 
    let HelloWorld = "Hello World!"; 


    //Operaciones
    let number1 = 35;
    let number2 = 33.4;
    let number3 = 1_000_000;

    let suma = number1 + number3;
    let resta = number3 - number1;
    let division = number3 / number1;
    let multiplicacion = number1 * number3;
    let modulo = number1 % number3;

    if (resta > 0){
        printf("The number is positive");
    }
    else if (resta < 0){
        printf("The number is negative");
    }
    else {
        printf("The number is 0");
    }
}

    func printf(example: String) {
        //Code...
    }

```

## Testing
Inside the main directory two important files can be found, "archivo.pdm" and
"archivoFalla.pdm", this are the two example file, the first one is expected to have a
succesful tokenization process, while the second one is expected to fail.
